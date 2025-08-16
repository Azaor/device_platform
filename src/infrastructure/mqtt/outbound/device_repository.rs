use std::collections::HashMap;

use rumqttc::{AsyncClient};
use uuid::Uuid;

use crate::{application::ports::outbound::device_repository::{CreateDeviceRepository, DeleteDeviceRepository, DeviceRepositoryError, UpdateDeviceRepository}, domain::device::Device, infrastructure::mqtt::mqtt_messages::{self, MqttActionType, MqttEventEmittable}};

#[derive(Debug)]
pub struct MqttDeviceRepository {
    mqtt_client: AsyncClient,
    device_topic: String
}

impl MqttDeviceRepository {
    pub fn new(mqtt_client: AsyncClient, device_topic: &str) -> Self {
        return MqttDeviceRepository { mqtt_client, device_topic: device_topic.to_string() }
    }

}

impl CreateDeviceRepository for MqttDeviceRepository {
    async fn create(&self, device: &Device) -> Result<(), DeviceRepositoryError> {
        let events_serialized: HashMap<String, MqttEventEmittable> = device.events().iter().map(|(k, v)| (k.to_string(), MqttEventEmittable::from(v))).collect();
        let events = match serde_json::to_string(&events_serialized) {
            Ok(r) => r,
            Err(e) => {
                return Err(DeviceRepositoryError::InternalError(e.to_string()))
            }
        };
        let mqtt_payload = mqtt_messages::CreateDevicePayload {
            id: device.id().to_string(),
            physical_id: device.physical_id().to_string(),
            user_id: device.user_id().to_string(),
            name: device.name().to_string(),
            events: events,
        };

        let message = match mqtt_messages::payload_to_mqtt_message(mqtt_payload, MqttActionType::Create) {
            Ok(r) => r,
            Err(e) => {
                return Err(DeviceRepositoryError::InternalError(e.to_string()));
            },
        };
        self.mqtt_client
            .publish(
                &self.device_topic,
                rumqttc::QoS::AtLeastOnce,
                true,
                message,
            )
            .await
            .map_err(|e| {
                println!("An error occured: {}", e);
                DeviceRepositoryError::InternalError(e.to_string())
            })?;

        return Ok(())
    }
}

impl UpdateDeviceRepository for MqttDeviceRepository {
    async fn update(&self, device: &Device) -> Result<(), DeviceRepositoryError> {
        let events_serialized: HashMap<String, MqttEventEmittable> = device.events().iter().map(|(k, v)| (k.to_string(), MqttEventEmittable::from(v))).collect();
        let events = match serde_json::to_string(&events_serialized) {
            Ok(r) => r,
            Err(e) => {
                return Err(DeviceRepositoryError::InternalError(e.to_string()))
            }
        };
        let payload = mqtt_messages::UpdateDevicePayload {
            id: device.id().to_string(),
            user_id: device.user_id().to_string(),
            physical_id: device.physical_id().to_string(),
            name: device.name().to_string(),
            events
        };

        let message = match mqtt_messages::payload_to_mqtt_message(payload, MqttActionType::Update) {
            Ok(r) => r,
            Err(e) => {
                return Err(DeviceRepositoryError::InternalError(e.to_string()));
            },
        };
        self.mqtt_client
            .publish(
                &self.device_topic,
                rumqttc::QoS::AtLeastOnce,
                true,
                message,
            )
            .await
            .map_err(|e| DeviceRepositoryError::InternalError(e.to_string()))?;

        return Ok(())
    }
}

impl DeleteDeviceRepository for MqttDeviceRepository {
    async fn delete_by_id(&self, id: Uuid) -> Result<(), DeviceRepositoryError> {
        let payload = mqtt_messages::DeleteDevicePayload {
            id: id.to_string()
        };
        let message = match mqtt_messages::payload_to_mqtt_message(payload, MqttActionType::Delete) {
            Ok(r) => r,
            Err(e) => {
                return Err(DeviceRepositoryError::InternalError(e.to_string()));
            },
        };
         self.mqtt_client
            .publish(
                &self.device_topic,
                rumqttc::QoS::AtLeastOnce,
                true,
                message,
            )
            .await
            .map_err(|e| DeviceRepositoryError::InternalError(e.to_string()))?;

        return Ok(())
    }
}