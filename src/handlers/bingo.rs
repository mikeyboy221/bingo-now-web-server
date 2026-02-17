use axum::{
    extract::State, 
    http::StatusCode, 
    response::{
        Response, 
        IntoResponse
    }, 
    Json
};
use uuid::Uuid;
use super::super::AppState;
use crate::bingo::{
    game::{Game, GameType, WinSubmission, Winner},
    patterns,
    events
};
use std::sync::Arc;
use chrono::DateTime;
use serde::{Serialize, Deserialize};


pub async fn connnect() {

}

pub async fn disconnect() {

}

#[derive(Serialize, Deserialize)]
pub struct SubmitWinRequest {
    user_id: u32,
    timestamp: f64,
    call: u8,
    pattern: Vec<u8>,
}

#[axum_macros::debug_handler]
pub async fn submit_win(
    State(app_state): State<AppState>,
    Json(payload): Json<SubmitWinRequest>, 
) -> Response {
    let timestamp_millis = payload.timestamp as i64;
    if DateTime::from_timestamp_millis(timestamp_millis).is_none() {
        tracing::warn!("Invalid timestamp: {}", payload.timestamp);
        return (StatusCode::BAD_REQUEST, "Invalid timestamp").into_response();
    }

    let win_submission = WinSubmission {
        user_id: payload.user_id,
        timestamp_millis: timestamp_millis,
        call: payload.call,
        pattern: payload.pattern
    };

    match app_state.win_tx.send(win_submission) {
        Ok(_) => StatusCode::OK.into_response(),
        Err(_) => {
            tracing::warn!("Failed to send win submission");
            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }
}

#[derive(Clone, Serialize)]
pub struct PollResponse {
    game_id: Uuid,
    game_type: GameType,
    numbers: Arc<Vec<u8>>,
    patterns: Arc<patterns::GamePatterns>,
    events: Vec<events::Event>,
    winners: Vec<Vec<Winner>>,
    current_call_index: u8,
    last_call_time: i64,
    current_round_index: u8,
}

impl From<&Game> for PollResponse {
    fn from(game: &Game) -> Self {
        PollResponse {
            game_id: game.game_id,
            game_type: game.game_type,
            numbers: Arc::clone(&game.numbers),
            patterns: Arc::clone(&game.patterns),
            events: game.events.clone(),
            winners: game.winners.clone(),
            current_call_index: game.current_call_index,
            last_call_time: game.last_call_time,
            current_round_index: game.current_round_index,
        }
    }
}

pub async fn poll(State(app_state): State<AppState>) -> Json<PollResponse> {
    let game_state = app_state.game.load();
    Json(PollResponse::from(&**game_state))

}

