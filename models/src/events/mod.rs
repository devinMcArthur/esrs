use thiserror::Error;

pub mod jobsite;

#[derive(Error, Debug)]
pub enum EventParseError {
    #[error("Event data is missing")]
    MissingEventData,
    #[error("Failed to deserialize event data: {0}")]
    DeserializationError(#[from] serde_json::Error),
    #[error("Unknown event type: {0}")]
    UnknownEventType(String),
}
