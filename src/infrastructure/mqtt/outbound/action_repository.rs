use rumqttc::AsyncClient;

use crate::{
    application::ports::outbound::action_repository::{
        ActionRepositoryError, CreateActionRepository,
    },
    domain::action::{action::Action, action_format::ActionFormat},
    infrastructure::mqtt::mqtt_messages::{self, MqttActionType},
};

#[derive(Debug)]
pub struct MqttActionRepository {
    mqtt_client: AsyncClient,
    action_topic: String,
}

impl MqttActionRepository {
    pub fn new(mqtt_client: AsyncClient, action_topic: &str) -> Self {
        return MqttActionRepository {
            mqtt_client,
            action_topic: action_topic.to_string(),
        };
    }
}

impl CreateActionRepository for MqttActionRepository {
    async fn create_action(
        &self,
        action: Action,
        action_format: &ActionFormat,
    ) -> Result<(), ActionRepositoryError> {
        let payload = mqtt_messages::CreateActionPayload {
            device_id: action.device_id.to_string(),
            device_action_name: action.action_name.to_string(),
            timestamp: action.timestamp.to_rfc3339(),
            action_data: ActionFormat::encode_event(action_format, action.payload)?,
        };
        let message = match mqtt_messages::payload_to_mqtt_message(payload, MqttActionType::Create)
        {
            Ok(r) => r,
            Err(e) => {
                return Err(ActionRepositoryError::RepositoryError(format!(
                    "Invalid data in action: {}",
                    e
                )));
            }
        };

        self.mqtt_client
            .publish(
                &self.action_topic,
                rumqttc::QoS::AtLeastOnce,
                false,
                message,
            )
            .await
            .map_err(|_| {
                ActionRepositoryError::RepositoryError("Failed to publish event".to_string())
            })?;
        Ok(())
    }
}
