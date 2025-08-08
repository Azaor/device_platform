use std::{collections::HashSet, sync::Arc};

use crate::{
    application::ports::{
        inbound::event_service::{EventService, EventServiceError},
        outbound::event_repository::{ CreateEventRepository, EventRepositoryError, GetEventRepository},
    },
    domain::{device::EventFormat, event::Event},
};

#[derive(Debug)]
pub struct ManageEventService<C: CreateEventRepository, G: GetEventRepository> {
    pub create_repo: Arc<C>,
    pub get_repo: Arc<G>,
}

impl<C: CreateEventRepository, G: GetEventRepository> EventService for ManageEventService<C, G> {
    async fn handle_event(&self, event: Event, event_format: &EventFormat) -> Result<(), EventServiceError> {
        if let Err(e) = self.create_repo.create_event(event.clone(), event_format).await {
            return Err(match e {
                EventRepositoryError::RepositoryError(msg) => EventServiceError::InternalError(msg),
                EventRepositoryError::ValidationError(msg) => EventServiceError::InvalidInput(msg),
            });
        }
        Ok(())
    }

    async fn get_events(&self, device_physical_id: &str) -> Result<Vec<Event>, EventServiceError> {
        let mut result = Vec::new();
        match self.get_repo.get_events(device_physical_id).await {
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
