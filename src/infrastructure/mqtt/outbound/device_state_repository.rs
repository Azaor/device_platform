use rumqttc::{AsyncClient, ClientError};
use uuid::Uuid;

use crate::{
    application::ports::outbound::{device_state_repository::{CreateDeviceStateRepository, DeleteDeviceStateRepository, DeviceStateRepositoryError, UpdateDeviceStateRepository}},
    domain::state::DeviceState,
};

pub struct MqttDeviceStateRepository {
    mqtt_client: AsyncClient,
    device_state_topic: String,
}

impl MqttDeviceStateRepository {
    pub async fn new(
        mqtt_client: AsyncClient,
        device_state_topic: &str,
        device_state_topic_response: &str,
    ) -> Result<Self, ClientError> {
        mqtt_client
            .subscribe(device_state_topic_response, rumqttc::QoS::AtLeastOnce)
            .await?;
        return Ok(MqttDeviceStateRepository {
            mqtt_client,
            device_state_topic: device_state_topic.to_string(),
        });
    }
}

impl CreateDeviceStateRepository for MqttDeviceStateRepository {
    async fn create(&self, device_state: &DeviceState) -> Result<(), DeviceStateRepositoryError> {
        let payload = serde_json::to_string(device_state)
            .map_err(|_| DeviceStateRepositoryError::InternalError)?;
        self.mqtt_client
            .publish(
                &self.device_state_topic,
                rumqttc::QoS::AtLeastOnce,
                false,
                payload,
            )
            .await
            .map_err(|_| DeviceStateRepositoryError::InternalError)?;
        Ok(())
    }
}

impl DeleteDeviceStateRepository for MqttDeviceStateRepository {
    async fn delete_by_id(&self, id: Uuid) -> Result<(), DeviceStateRepositoryError> {
        let payload = serde_json::json!({ "device_id": id }).to_string();
        self.mqtt_client
            .publish(
                &self.device_state_topic,
                rumqttc::QoS::AtLeastOnce,
                false,
                payload,
            )
            .await
            .map_err(|_| DeviceStateRepositoryError::InternalError)?;
        Ok(())
    }
}

impl UpdateDeviceStateRepository for MqttDeviceStateRepository {
    async fn update(&self, device_state: &DeviceState) -> Result<(), DeviceStateRepositoryError> {
        let payload = serde_json::to_string(device_state)
            .map_err(|_| DeviceStateRepositoryError::InternalError)?;
        self.mqtt_client
            .publish(
                &self.device_state_topic,
                rumqttc::QoS::AtLeastOnce,
                false,
                payload,
            )
            .await
            .map_err(|_| DeviceStateRepositoryError::InternalError)?;
        Ok(())
    }
}

