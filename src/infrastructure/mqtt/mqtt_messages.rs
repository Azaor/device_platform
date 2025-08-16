use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
#[cfg(feature = "mqtt_inbound")]
use tracing::warn;

use crate::{domain::device::{EventDataType, EventEmittable, EventFormat}};
#[cfg(feature = "mqtt_inbound")]
use crate::infrastructure::mqtt::inbound::error::HandlerError;

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
    pub physical_id: String,
    pub user_id: String,
    pub name: String,
    pub events: String,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateDevicePayload {
    pub id: String,
    pub user_id: String,
    pub physical_id: String,
    pub name: String,
    pub events: String,
}

#[derive(Serialize, Deserialize)]
pub struct DeleteDevicePayload {
    pub id: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateEventPayload {
    pub device_physical_id: String,
    pub device_event_name: String,
    pub timestamp: String,
    pub event_data: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateDeviceStatePayload {
    pub device_id: String,
    pub last_update: String,
    pub values: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateDeviceStatePayload {
    pub device_id: String,
    pub last_update: String,
    pub values: HashMap<String, Value>,
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

#[derive(Serialize, Deserialize)]
pub struct MqttEventEmittable {
    format: String,
    payload: HashMap<String, EventDataType>
}

impl TryFrom<MqttEventEmittable> for EventEmittable {
    type Error = String;

    fn try_from(value: MqttEventEmittable) -> Result<Self, Self::Error> {
        let format = EventFormat::try_from(value.format.as_str())?;
        Ok(EventEmittable::new(format, value.payload))
    }
}

impl From<&EventEmittable> for MqttEventEmittable {
    fn from(value: &EventEmittable) -> Self {
        MqttEventEmittable {
            format: value.format().to_string(),
            payload: value.payload().clone(),
        }
    }
}
#[cfg(feature = "mqtt_inbound")]
pub fn deserialize_event(events: &str) -> Result<HashMap<String, EventEmittable>, HandlerError> {
    let events_deserialized: HashMap<String, MqttEventEmittable> = match serde_json::from_str(&events) {
        Ok(e) => e,
        Err(e) => {
            warn!(
                result = "warn",
                details = format!("Invalid events : {}", e.to_string())
            );
            return Err(HandlerError::ClientError(e.to_string()))
        }
    };
    let mut events = HashMap::new();
    for (key, val) in events_deserialized {
        let event = match EventEmittable::try_from(val) {
            Ok(v) => v,
            Err(e) => {
                warn!(
                    result = "warn",
                    details = format!("Invalid events : {}", e.to_string())
                );
                return Err(HandlerError::ClientError(e.to_string()))
            },
        };
        events.insert(key, event);
    }
    return Ok(events)
}