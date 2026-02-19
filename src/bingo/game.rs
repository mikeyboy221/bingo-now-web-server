use std::{sync::Arc, time::Duration};
use arc_swap::ArcSwap;
use chrono::Utc;
use tokio::{
    sync::mpsc,
    time::sleep,
};
use serde::Serialize;
use tracing;
use uuid::Uuid;
use rand::seq::SliceRandom;

use crate::bingo::{events, patterns};

const INTERMISSION_DURATION_IN_SECS: u64 = 10; 
const CALL_DURATION_IN_SECS: u64 = 8;
const ROUND_WIN_DURATION_IN_SECS: u64 = 16;
const NEW_ROUND_DURATION_IN_SECS: u64 = 6;
const END_OF_GAME_DURATION_IN_SECS: u64 = 16;

#[derive(Clone, Serialize)]
pub struct Winner {
    user_id: u32,
    winning_pattern: Vec<u8>,
    position: u8,
}

#[derive(Debug, Clone)]
pub struct WinSubmission {
    pub user_id: u32,
    pub timestamp_millis: i64,
    pub call: u8,
    pub pattern: Vec<u8>,
}

#[derive(Clone, Copy, Serialize)]
pub enum GameType {
    Classic,
    British,
    Picture,
}

impl GameType {
    fn random() -> Self {
        // match rand::random_range(0..2) {
        //     0 => Self::Classic,
        //     1 => Self::British,
        //     _ => Self::Picture,
        // }

        Self::British
    }
}

#[derive(Clone)]
pub struct Game {
    pub game_id: Uuid,
    pub game_type: GameType,
    pub numbers: Arc<Vec<u8>>,
    pub patterns: Arc<patterns::GamePatterns>,
    pub events: Vec<events::Event>,
    pub winners: Vec<Vec<Winner>>,
    pending_winners: Vec<WinSubmission>,
    pub current_call_index: u8,
    pub last_call_time: i64,
    pub current_round_index: u8,
}

impl Game {
    pub fn increment_round(&mut self) -> bool {
        let game_has_ended = self.current_round_index as usize == self.patterns.len();
        if game_has_ended {
            return false;
        }

        self.current_round_index += 1;
        true
    }

    pub fn should_increment_round(&self) -> bool {
        self.winners[self.current_round_index as usize - 1].len() > 0
    }

    pub fn increment_call(&mut self) -> bool {
        let game_has_ended = self.current_call_index as usize == self.numbers.len();
        if game_has_ended {
            return false;
        }
        
        self.current_call_index += 1;
        self.last_call_time = Utc::now().timestamp_millis();

        true
    }

    pub fn process_win_submission(&mut self, submission: WinSubmission) -> bool {
        tracing::info!("{:?}", submission);

        let valid_submission_call = submission.call == self.current_call_index;
        if !valid_submission_call {
            tracing::debug!("Submission call invalid, submitted with call {}, game call {}", submission.call, self.current_call_index);
            return false;
        }
        
        let millis_elapsed_since_call = submission.timestamp_millis - self.last_call_time;
        let secs_elapsed_since_call = millis_elapsed_since_call as f64 / 1000.0;
        let valid_submission_time = millis_elapsed_since_call < (CALL_DURATION_IN_SECS * 1000) as i64;
        if !valid_submission_time {
            tracing::debug!(
                "Submission time invalid, {:.2}s elapsed since call, call duration {}s", 
                secs_elapsed_since_call,
                CALL_DURATION_IN_SECS
            );
            return false;
        }


        if self.pending_winners.iter().any(|s| s.user_id == submission.user_id) {
            tracing::debug!("User has already submitted win");
            return false;
        }

        tracing::info!("Submitted win for user {}, answered in {:.2}s", submission.user_id, secs_elapsed_since_call);
        self.pending_winners.push(submission);
        true
    }

    pub fn process_winners(&mut self) {
        if self.pending_winners.is_empty() {
            return;
        }

        self.pending_winners.sort_by_key(|s| s.timestamp_millis);

        let round_winners: Vec<Winner> = self.pending_winners
            .iter()
            .enumerate()
            .map(|(index, submission)| Winner {
                user_id: submission.user_id,
                winning_pattern: submission.pattern.clone(),
                position: index as u8 + 1 
            })
        .collect();
        let number_of_winners = round_winners.len();

        self.winners[self.current_round_index as usize - 1] = round_winners;
        self.pending_winners.clear();

        tracing::debug!("Processed winners, {} winner(s)", number_of_winners);
    }

    pub fn has_pending_winners(& self) -> bool {
        !self.pending_winners.is_empty()
    }

    pub fn ended(&self) {

    }
}

pub fn new() -> Game {
    let random_game_type =  GameType::random();
    let patterns = patterns::get_game_patterns(&random_game_type);
    let number_of_rounds = patterns.len();
    let mut numbers: Vec<u8> = match random_game_type {
        GameType::Classic => (1..=75).collect(),
        GameType::British => (1..=90).collect(),
        GameType::Picture => (1..=60).collect()
    };
    numbers.shuffle(&mut rand::rng());

    Game {
        game_id: Uuid::new_v4(),
        game_type: random_game_type,
        numbers: Arc::new(numbers),
        patterns: Arc::new(patterns),
        winners: vec![Vec::new(); number_of_rounds],
        events: vec![events::new(events::EventType::Intermission, INTERMISSION_DURATION_IN_SECS)],
        pending_winners: Vec::new(),
        current_call_index: 0,
        last_call_time: Utc::now().timestamp_millis(),
        current_round_index: 1,
    }
}

pub async fn run(
    mut win_receiver: mpsc::UnboundedReceiver<WinSubmission>, 
    game_state: Arc<ArcSwap<Game>>, 
) {
    tracing::info!("Starting game loop");

    loop {
        let mut game = new();
        tracing::info!("New game!");

        game_state.store(Arc::new(game.clone()));
        sleep(Duration::from_secs(INTERMISSION_DURATION_IN_SECS)).await;

        // new round (round 1 is a new round)
        game.events.push(events::new(events::EventType::NewRound, NEW_ROUND_DURATION_IN_SECS));
        game_state.store(Arc::new(game.clone()));
        sleep(Duration::from_secs(NEW_ROUND_DURATION_IN_SECS)).await;

        loop {
            // increment call
            if !game.increment_call() {
                tracing::warn!("Game ended, ran out of numbers on round {}", game.current_round_index);
                break;
            }

            game.events.push(events::new(events::EventType::NewCall, CALL_DURATION_IN_SECS));
            game_state.store(Arc::new(game.clone()));

            let deadline = tokio::time::Instant::now() + Duration::from_secs(CALL_DURATION_IN_SECS);
            loop {
                match tokio::time::timeout_at(deadline, win_receiver.recv()).await {
                    Ok(Some(submission)) => { game.process_win_submission(submission); },
                    Ok(None) => break,
                    Err(_) => break
                }
            }

            // process winners
            if game.has_pending_winners() {
                game.process_winners();

                game.events.push(events::new(events::EventType::RoundWin, ROUND_WIN_DURATION_IN_SECS));
                game_state.store(Arc::new(game.clone()));
                sleep(Duration::from_secs(ROUND_WIN_DURATION_IN_SECS)).await;
            }

            // increment round if winners
            if game.should_increment_round() {
                if !game.increment_round() {
                    break;
                };

                game.events.push(events::new(events::EventType::NewRound, NEW_ROUND_DURATION_IN_SECS));
                game_state.store(Arc::new(game.clone()));
                sleep(Duration::from_secs(NEW_ROUND_DURATION_IN_SECS)).await
            }
        }

        game.ended();

        game.events.push(events::new(events::EventType::EndOfGame, END_OF_GAME_DURATION_IN_SECS));
        game_state.store(Arc::new(game.clone()));
        sleep(Duration::from_secs(END_OF_GAME_DURATION_IN_SECS)).await;
    }
}
