use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct MqttMessage<S: Serialize> {
    pub action_type: MqttActionType,
    pub payload: S,
}

#[derive(Serialize, Deserialize)]
pub enum MqttActionType {
    Create,
    Update,
    Delete,
}

#[derive(Serialize, Deserialize)]
pub struct CreateDevicePayload {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub event_format: String,
    pub event_data: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateDevicePayload {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub event_format: String,
    pub event_data: String,
}

#[derive(Serialize, Deserialize)]
pub struct DeleteDevicePayload {
    pub id: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateEventPayload {
    pub device_id: String,
    pub timestamp: String,
    pub event_data: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateDeviceStatePayload {
    pub device_id: String,
    pub last_update: String,
    pub values: HashMap<String, String>,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateDeviceStatePayload {
    pub device_id: String,
    pub last_update: String,
    pub values: HashMap<String, String>,
}

#[derive(Serialize, Deserialize)]
pub struct DeleteDeviceStatePayload {
    pub device_id: String,
}

#[cfg(feature = "mqtt_outbound")]
pub fn payload_to_mqtt_message<S: Serialize>(payload: S, action_type: MqttActionType) -> Result<Vec<u8>, serde_json::Error> {
    Ok(serde_json::to_vec(&MqttMessage {
            action_type,
            payload: payload
    })?)
}
