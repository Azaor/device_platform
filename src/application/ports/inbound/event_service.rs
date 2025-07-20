use uuid::Uuid;

use crate::domain::{device::EventFormat, event::Event};

pub enum EventServiceError {
    InvalidInput(String),
    InternalError(String),
}

pub trait EventService {
    async fn handle_event(&self, event: Event, event_format: &EventFormat) -> Result<(), EventServiceError>;
    async fn get_events(&self, device_id: &Uuid) -> Result<Vec<Event>, EventServiceError>;
}