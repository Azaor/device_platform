use uuid::Uuid;

use crate::domain::event::Event;

pub enum EventRepositoryError {
    RepositoryError(String),
    ValidationError(String),
}


pub trait GetEventRepository: Send + Sync {
    fn get_events(&self, device_id: &Uuid) -> impl Future<Output = Result<Vec<Event>, EventRepositoryError>> + Send;
}

pub trait CreateEventRepository: Send + Sync {
    fn create_event(&self, event: Event) -> impl Future<Output = Result<(), EventRepositoryError>> + Send;
}