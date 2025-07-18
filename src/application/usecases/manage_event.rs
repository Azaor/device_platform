use std::collections::HashSet;

use crate::{
    application::ports::{
        inbound::event_service::{EventService, EventServiceError},
        outbound::event_repository::{ EventRepository, EventRepositoryError},
    },
    domain::event::Event,
};

pub struct ManageEventService<E: EventRepository> {
    pub event_repository: E,
}

impl<E: EventRepository> EventService for ManageEventService<E> {
    async fn handle_event(&self, event: Event) -> Result<(), EventServiceError> {
        if let Err(e) = self.event_repository.create_event(event.clone()).await {
            return Err(match e {
                EventRepositoryError::RepositoryError(msg) => EventServiceError::InternalError(msg),
                EventRepositoryError::ValidationError(msg) => EventServiceError::InvalidInput(msg),
            });
        }
        Ok(())
    }

    async fn get_events(&self, device_id: &uuid::Uuid) -> Result<Vec<Event>, EventServiceError> {
        let mut result = Vec::new();
        match self.event_repository.get_events(device_id).await {
            Ok(events) => result.push(events),
            Err(EventRepositoryError::RepositoryError(msg)) => {
                return Err(EventServiceError::InternalError(msg));
            }
            Err(EventRepositoryError::ValidationError(msg)) => {
                return Err(EventServiceError::InvalidInput(msg));
            }
        }
        let mut events = HashSet::new();
        for repo_events in result {
            for event in repo_events {
                events.insert(event);
            }
        }
        Ok(events.into_iter().collect())
    }
}
