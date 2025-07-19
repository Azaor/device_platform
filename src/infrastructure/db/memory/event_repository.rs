use std::{collections::HashMap, sync::Mutex};

use uuid::Uuid;

use crate::{application::ports::outbound::event_repository::{CreateEventRepository, EventRepositoryError, GetEventRepository}, domain::event::Event};

pub struct InMemoryEventRepository {
    events: Mutex<HashMap<Uuid, Vec<Event>>>
}

impl InMemoryEventRepository {
    pub fn new() -> Self {
        return InMemoryEventRepository { events: Mutex::new(HashMap::new()) }
    }
}
impl CreateEventRepository for InMemoryEventRepository {
    async fn create_event(&self, event: Event) -> Result<(), EventRepositoryError> {
        let mut events = self.events.lock().unwrap();
        match events.get_mut(&event.device_id) {
            Some(device_events) => device_events.push(event),
            None => {
                events.insert(event.device_id, vec![event]);
            },
        }
        return Ok(())
    }
}

impl GetEventRepository for InMemoryEventRepository {
    async fn get_events(&self, device_id: &Uuid) -> Result<Vec<Event>, EventRepositoryError> {
        let events = self.events.lock().unwrap();
        let events_found = match events.get(device_id) {
            Some(device_events) => device_events,
            None => &Vec::new(),
        };
        return Ok(events_found.clone());
    }
}