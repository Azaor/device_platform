use rumqttc::AsyncClient;
use uuid::Uuid;

use crate::{
    application::ports::outbound::device_state_repository::{
        CreateDeviceStateRepository, DeleteDeviceStateRepository, DeviceStateRepositoryError,
        UpdateDeviceStateRepository,
    },
    domain::state::DeviceState,
    infrastructure::mqtt::mqtt_messages::{self, MqttActionType},
};

pub struct MqttDeviceStateRepository {
    mqtt_client: AsyncClient,
    device_state_topic: String,
}

impl MqttDeviceStateRepository {
    pub fn new(mqtt_client: AsyncClient, device_state_topic: &str) -> Self {
        return MqttDeviceStateRepository {
            mqtt_client,
            device_state_topic: device_state_topic.to_string(),
        };
    }
}

impl CreateDeviceStateRepository for MqttDeviceStateRepository {
    async fn create(&self, device_state: &DeviceState) -> Result<(), DeviceStateRepositoryError> {
        let payload = mqtt_messages::CreateDeviceStatePayload {
            device_id: device_state.device_id.to_string(),
            last_update: device_state.last_update.to_rfc3339(),
            values: device_state.values.clone(),
        };

        let message = match mqtt_messages::payload_to_mqtt_message(payload, MqttActionType::Create){
            Ok(r) => r,
            Err(e) => return Err(DeviceStateRepositoryError::InternalError(e.to_string())),
        };
        self.mqtt_client
            .publish(
                &self.device_state_topic,
                rumqttc::QoS::AtLeastOnce,
                false,
                message,
            )
            .await
            .map_err(|e| DeviceStateRepositoryError::InternalError(e.to_string()))?;
        Ok(())
    }
}

impl DeleteDeviceStateRepository for MqttDeviceStateRepository {
    async fn delete_by_id(&self, id: Uuid) -> Result<(), DeviceStateRepositoryError> {
        let payload = mqtt_messages::DeleteDeviceStatePayload {
            device_id: id.to_string(),
        };
        let message = match mqtt_messages::payload_to_mqtt_message(payload, MqttActionType::Delete){
            Ok(r) => r,
            Err(e) => return Err(DeviceStateRepositoryError::InternalError(e.to_string())),
        };
        self.mqtt_client
            .publish(
                &self.device_state_topic,
                rumqttc::QoS::AtLeastOnce,
                false,
                message,
            )
            .await
            .map_err(|e| DeviceStateRepositoryError::InternalError(e.to_string()))?;
        Ok(())
    }
}

impl UpdateDeviceStateRepository for MqttDeviceStateRepository {
    async fn update(&self, device_state: &DeviceState) -> Result<(), DeviceStateRepositoryError> {
        let payload = mqtt_messages::UpdateDeviceStatePayload {
            device_id: device_state.device_id.to_string(),
            last_update: device_state.last_update.to_rfc3339(),
            values: device_state.values.clone(),
        };

        let message = match mqtt_messages::payload_to_mqtt_message(payload, MqttActionType::Update){
            Ok(r) => r,
            Err(e) => return Err(DeviceStateRepositoryError::InternalError(e.to_string())),
        };

        self.mqtt_client
            .publish(
                &self.device_state_topic,
                rumqttc::QoS::AtLeastOnce,
                false,
                message,
            )
            .await
            .map_err(|e| DeviceStateRepositoryError::InternalError(e.to_string()))?;
        Ok(())
    }
}
