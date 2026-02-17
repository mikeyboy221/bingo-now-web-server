use chrono::Utc;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub enum EventType {
    Intermission,
    NewCall,
    RoundWin,
    NewRound,
    EndOfGame,
}

#[derive(Clone, Serialize)]
pub struct Event {
    event_type: EventType,
    duration: u8,
    start_time:i64 
}

pub fn new(event_type: EventType, event_duration: u64) -> Event {
    Event {
        event_type: event_type,
        duration: event_duration as u8,
        start_time: Utc::now().timestamp_millis()
    }
}
