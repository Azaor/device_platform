use uuid::Uuid;

use crate::{application::ports::outbound::event_repository::{EventRepository, EventRepositoryError}, domain::event::Event};

pub struct ReqwestEventRepository {
    base_url: String,
}

impl ReqwestEventRepository {
    pub fn new(base_url: String) -> Self {
        Self { base_url }
    }
}

impl EventRepository for ReqwestEventRepository {
    fn create_event(&self, event: Event) -> Result<(), EventRepositoryError> {
        // Send a POST request to the base URL with the event data
        let client = reqwest::blocking::Client::new();
        let response = client.post(&self.base_url)
            .json(&event)
            .send()
            .map_err(|e| EventRepositoryError::RepositoryError(e.to_string()))?;
        if response.status().is_success() {
            Ok(())
        } else {
            Err(EventRepositoryError::RepositoryError(format!(
                "Failed to create event: {}",
                response.status()
            )))
        }
    }

    fn get_events(&self, device_id: &Uuid) -> Result<Vec<Event>, EventRepositoryError> {
        // Implementation for retrieving events using reqwest
        let client = reqwest::blocking::Client::new();
        let url = format!("{}/events/{}", self.base_url, device_id);
        let response = client.get(&url)
            .send()
            .map_err(|e| EventRepositoryError::RepositoryError(e.to_string()))?;
        if response.status().is_success() {
            let events: Vec<Event> = response.json().map_err(|e| EventRepositoryError::RepositoryError(e.to_string()))?;
            return Ok(events);
        }
        Err(EventRepositoryError::RepositoryError(format!(
            "Failed to retrieve events: {}",
            response.status()
        )))
    }
}