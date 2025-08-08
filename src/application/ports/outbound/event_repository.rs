use uuid::Uuid;

use crate::domain::{device::{EventFormat, EventFormatError}, event::Event};

pub enum EventRepositoryError {
    RepositoryError(String),
    ValidationError(String),
}

pub trait GetEventRepository: Send + Sync {
    fn get_events(&self, device_physical_id: &str) -> impl Future<Output = Result<Vec<Event>, EventRepositoryError>> + Send;
}

pub trait CreateEventRepository: Send + Sync {
    fn create_event(&self, event: Event, event_format: &EventFormat) -> impl Future<Output = Result<(), EventRepositoryError>> + Send;
}

impl From<EventFormatError> for EventRepositoryError {
    fn from(value: EventFormatError) -> Self {
        match value {
            EventFormatError::UnsupportedFormat(e) => EventRepositoryError::RepositoryError(format!("Unsupported format: {}", e)),
        }
    }
}
