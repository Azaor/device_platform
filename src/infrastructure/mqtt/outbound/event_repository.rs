use rumqttc::AsyncClient;

use crate::{
    application::ports::outbound::event_repository::{CreateEventRepository, EventRepositoryError},
    domain::{device::EventFormat, event::Event},
    infrastructure::mqtt::mqtt_messages::{self, MqttActionType},
};

#[derive(Debug)]
pub struct MqttEventRepository {
    mqtt_client: AsyncClient,
    event_topic: String,
}

impl MqttEventRepository {
    pub fn new(mqtt_client: AsyncClient, event_topic: &str) -> Self {
        return MqttEventRepository {
            mqtt_client,
            event_topic: event_topic.to_string(),
        };
    }
}

impl CreateEventRepository for MqttEventRepository {
    async fn create_event(
        &self,
        event: Event,
        event_format: &EventFormat,
    ) -> Result<(), EventRepositoryError> {
        let payload = mqtt_messages::CreateEventPayload {
            device_physical_id: event.device_physical_id.to_string(),
            timestamp: event.timestamp.to_rfc3339(),
            event_data: EventFormat::encode_event(event_format, event.payload)?,
        };
        let message = match mqtt_messages::payload_to_mqtt_message(payload, MqttActionType::Create)
        {
            Ok(r) => r,
            Err(e) => {
                return Err(EventRepositoryError::RepositoryError(format!(
                    "Invalid data in event: {}",
                    e
                )));
            }
        };

        self.mqtt_client
            .publish(&self.event_topic, rumqttc::QoS::AtLeastOnce, false, message)
            .await
            .map_err(|_| {
                EventRepositoryError::RepositoryError("Failed to publish event".to_string())
            })?;
        Ok(())
    }
}
