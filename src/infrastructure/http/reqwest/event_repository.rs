use crate::{
    application::ports::outbound::event_repository::{
        CreateEventRepository, EventRepositoryError, GetEventRepository,
    },
    domain::{
        device::{EventFormat, EventFormatError},
        event::Event,
    }, infrastructure::http::reqwest::types::{EventToReceive, EventToSend},
};

#[derive(Debug)]
pub struct ReqwestEventRepository {
    base_url: String,
    create_path: String,
    get_path: String,
}

impl ReqwestEventRepository {
    pub fn new(base_url: &str, create_path: &str, get_path: &str) -> Self {
        Self { base_url: base_url.to_string(), create_path: create_path.to_string(), get_path: get_path.to_string() }
    }
}

impl CreateEventRepository for ReqwestEventRepository {
    async fn create_event(
        &self,
        event: Event,
        event_format: &EventFormat,
    ) -> Result<(), EventRepositoryError> {
        let url = format!("{}{}", self.base_url, self.create_path);

        let event_id = event.id.to_string();
        let device_id = event.device_id.to_string();
        let timestamp = event.timestamp.to_rfc3339();
        let data = event_format
            .encode_event(event.payload)
            .map_err(|err| match err {
                EventFormatError::UnsupportedFormat(e) => {
                    EventRepositoryError::ValidationError(format!("invalid format, {}", e))
                }
            })?;

        let event_to_send = EventToSend {
            id: event_id,
            device_id,
            timestamp,
            payload: data,
        };

        let client = reqwest::Client::new();

        let res = client
            .post(&url)
            .json(&event_to_send)
            .header("Content-Type", "application/json")
            .send()
            .await
            .map_err(|e| EventRepositoryError::RepositoryError(e.to_string()))?;

        if res.status().is_success() {
            Ok(())
        } else {
            Err(EventRepositoryError::RepositoryError(
                res.status().to_string(),
            ))
        }
    }
}

impl GetEventRepository for ReqwestEventRepository {
    async fn get_events(&self, device_id: &uuid::Uuid) -> Result<Vec<Event>, EventRepositoryError> {
        let url = format!("{}{}/{}", self.base_url, self.get_path, device_id);

        let client = reqwest::Client::new();

        let res = client
            .get(&url)
            .header("Content-Type", "application/json")
            .send()
            .await
            .map_err(|e| EventRepositoryError::RepositoryError(e.to_string()))?;

        if res.status().is_success() {
            let events_to_receive = res
                .json::<Vec<EventToReceive>>()
                .await
                .map_err(|e| EventRepositoryError::RepositoryError(e.to_string()))?;
        
            // Map EventToReceive to Event
            let mut events = Vec::new();
            for event_to_receive in events_to_receive {
                events.push(event_to_receive.try_into()?);
            }
            Ok(events)
        } else {
            Err(EventRepositoryError::RepositoryError(
                res.status().to_string(),
            ))
        }
    }
}
