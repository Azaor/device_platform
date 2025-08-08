use std::fmt::{Display};

use uuid::Uuid;

use crate::domain::{device::EventFormat, event::Event};

#[derive(Debug)]
pub enum EventServiceError {
    InvalidInput(String),
    InternalError(String),
}

impl Display for EventServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventServiceError::InvalidInput(v) => f.write_str(&format!("Invalid input: {}", &v)),
            EventServiceError::InternalError(v) => f.write_str(&format!("Internal error: {}", &v)),
        }
    }
}

pub trait EventService {
    async fn handle_event(&self, event: Event, event_format: &EventFormat) -> Result<(), EventServiceError>;
    async fn get_events(&self, device_physical_id: &str) -> Result<Vec<Event>, EventServiceError>;
}