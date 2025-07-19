use rumqttc::AsyncClient;

use crate::{application::ports::outbound::event_repository::{CreateEventRepository, EventRepositoryError}, domain::event::Event};

pub struct MqttEventRepository {
    mqtt_client: AsyncClient,
    event_topic: String
}

impl MqttEventRepository {
    pub fn new(mqtt_client: AsyncClient, event_topic: &str, event_topic_response: &str) -> Self {
        mqtt_client.subscribe(event_topic_response, rumqttc::QoS::AtLeastOnce);
        return MqttEventRepository { mqtt_client, event_topic: event_topic.to_string() }
    }
}

impl CreateEventRepository for MqttEventRepository {
    async fn create_event(&self, event: Event) -> Result<(), EventRepositoryError> {
        let payload = serde_json::to_string(&event.payload)
            .map_err(|_| EventRepositoryError::ValidationError("Failed to serialize event".to_string()))?;
        self.mqtt_client
            .publish(
                &self.event_topic,
                rumqttc::QoS::AtLeastOnce,
                false,
                payload,
            )
            .await
            .map_err(|_| EventRepositoryError::RepositoryError("Failed to publish event".to_string()))?;
        Ok(())
    }
}