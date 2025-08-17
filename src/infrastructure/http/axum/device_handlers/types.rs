use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::domain::{action::action_emittable::ActionEmittable, event::event_emittable::EventEmittable};

pub struct CreateDeviceRequest {
    pub physical_id: String,
    pub user_id: Uuid,
    pub name: String,
    pub events: HashMap<String, EventEmittableSerializable>,
    pub actions: HashMap<String, ActionEmittableSerializable>,
}

impl TryFrom<Value> for CreateDeviceRequest {
    type Error = String;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let user_id_raw = value.get("user_id");
        let user_id = match user_id_raw {
            Some(id) => id
                .as_str()
                .and_then(|s| Uuid::parse_str(s).ok())
                .ok_or_else(|| String::from("Invalid user_id format"))?,
            None => return Err("Missing user_id".to_string()),
        };
        let physical_id = value
            .get("physical_id")
            .and_then(Value::as_str)
            .map(String::from)
            .ok_or_else(|| "Missing physical_id".to_string())?;
        let name = value
            .get("name")
            .and_then(Value::as_str)
            .map(String::from)
            .ok_or_else(|| "Missing name".to_string())?;
        let events_raw = value.get("events").and_then(Value::as_object);
        let mut events = HashMap::new();
        if let Some(data) = events_raw {
            for (key, value) in data.iter() {
                let event = EventEmittableSerializable {
                    format: value
                        .get("format")
                        .and_then(Value::as_str)
                        .map(String::from)
                        .ok_or_else(|| format!("Missing format for event {}", key))?,
                    payload: value
                        .get("payload")
                        .and_then(Value::as_object)
                        .ok_or_else(|| format!("Missing payload for event {}", key))?
                        .iter()
                        .map(|(k, v)| (k.clone(), v.as_str().unwrap_or("").to_string()))
                        .collect(),
                };
                events.insert(key.clone(), event);
            }
        }
        let actions_raw = value.get("actions").and_then(Value::as_object);
        let mut actions = HashMap::new();
        if let Some(data) = actions_raw {
            for (key, value) in data.iter() {
                let action = ActionEmittableSerializable {
                    format: value
                        .get("format")
                        .and_then(Value::as_str)
                        .map(String::from)
                        .ok_or_else(|| format!("Missing format for event {}", key))?,
                    payload: value
                        .get("payload")
                        .and_then(Value::as_object)
                        .ok_or_else(|| format!("Missing payload for event {}", key))?
                        .iter()
                        .map(|(k, v)| (k.clone(), v.as_str().unwrap_or("").to_string()))
                        .collect(),
                };
                actions.insert(key.clone(), action);
            }
        }
        Ok(CreateDeviceRequest {
            user_id,
            physical_id,
            name,
            events,
            actions
        })
    }
}

pub struct UpdateDeviceRequest {
    pub physical_id: Option<String>,
    pub name: Option<String>,
    pub events: Option<HashMap<String, EventEmittableSerializable>>,
    pub actions: Option<HashMap<String, ActionEmittableSerializable>>
}

impl TryFrom<Value> for UpdateDeviceRequest {
    type Error = String;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let physical_id = value
            .get("physical_id")
            .and_then(Value::as_str)
            .map(String::from);
        let name = value
            .get("name")
            .and_then(Value::as_str)
            .map(String::from);
        let events_raw = value.get("events").and_then(Value::as_object);
        let mut events = None;
        if let Some(data) = events_raw {
            let mut events_to_update = HashMap::new();
            for (key, value) in data.iter() {
                let event = EventEmittableSerializable {
                    format: value
                        .get("format")
                        .and_then(Value::as_str)
                        .map(String::from)
                        .ok_or_else(|| format!("Missing format for event {}", key))?,
                    payload: value
                        .get("payload")
                        .and_then(Value::as_object)
                        .ok_or_else(|| format!("Missing payload for event {}", key))?
                        .iter()
                        .map(|(k, v)| (k.clone(), v.as_str().unwrap_or("").to_string()))
                        .collect(),
                };
                events_to_update.insert(key.clone(), event);
            }
            events = Some(events_to_update)
        }
        let actions_raw = value.get("actions").and_then(Value::as_object);
        let mut actions = None;
        if let Some(data) = actions_raw {
            let mut actions_to_update = HashMap::new();
            for (key, value) in data.iter() {
                let event = ActionEmittableSerializable {
                    format: value
                        .get("format")
                        .and_then(Value::as_str)
                        .map(String::from)
                        .ok_or_else(|| format!("Missing format for action {}", key))?,
                    payload: value
                        .get("payload")
                        .and_then(Value::as_object)
                        .ok_or_else(|| format!("Missing payload for action {}", key))?
                        .iter()
                        .map(|(k, v)| (k.clone(), v.as_str().unwrap_or("").to_string()))
                        .collect(),
                };
                actions_to_update.insert(key.clone(), event);
            }
            actions = Some(actions_to_update)
        }
        Ok(UpdateDeviceRequest {
            physical_id,
            name,
            events,
            actions
        })
    }
}

#[derive(Serialize)]
pub struct DeviceResponse {
    pub id: Uuid,
    pub physical_id: String,
    pub user_id: Uuid,
    pub name: String,
    pub events: HashMap<String, EventEmittableSerializable>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventEmittableSerializable {
    pub format: String,
    pub payload: HashMap<String, String>,
}
impl From<EventEmittable> for EventEmittableSerializable {
    fn from(value: EventEmittable) -> Self {
        EventEmittableSerializable {
            format: value.format().to_string(),
            payload: value
                .payload()
                .iter()
                .map(|(k, v)| (k.clone(), v.to_string()))
                .collect(),
        }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct ActionEmittableSerializable {
    pub format: String,
    pub payload: HashMap<String, String>,
}
impl From<ActionEmittable> for ActionEmittableSerializable {
    fn from(value: ActionEmittable) -> Self {
        ActionEmittableSerializable {
            format: value.format().to_string(),
            payload: value
                .payload()
                .iter()
                .map(|(k, v)| (k.clone(), v.to_string()))
                .collect(),
        }
    }
}