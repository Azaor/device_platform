use std::{collections::HashMap, str::FromStr};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

use crate::{
    application::ports::outbound::{
        device_repository::DeviceRepositoryError,
        device_state_repository::DeviceStateRepositoryError,
        event_repository::EventRepositoryError,
    },
    domain::{
        action::{
            action_data_type::ActionDataType,
            action_emittable::ActionEmittable, action_format::ActionFormat,
        },
        device::Device,
        event::{
            event::Event, event_data_type::EventDataType, event_data_value::EventDataValue,
            event_emittable::EventEmittable, event_format::EventFormat,
        },
        state::DeviceState,
    },
};

#[derive(Serialize, Deserialize)]
pub struct DeviceToSend {
    pub id: String,
    pub physical_id: String,
    pub user_id: String,
    pub name: String,
    pub events: HashMap<String, EventEmittableToSend>,
    pub actions: HashMap<String, ActionEmittableToSend>,
}

impl From<Device> for DeviceToSend {
    fn from(device: Device) -> Self {
        let device_id = device.id().to_string();
        let user_id = device.user_id().to_string();
        let name = device.name().to_string();
        let events = device
            .events()
            .iter()
            .map(|(key, val)| return (key.clone(), EventEmittableToSend::from(val)))
            .collect();
        let actions = device
            .actions()
            .iter()
            .map(|(key, val)| return (key.clone(), ActionEmittableToSend::from(val)))
            .collect();
        DeviceToSend {
            id: device_id,
            physical_id: device.physical_id().to_string(),
            user_id,
            name,
            events,
            actions,
        }
    }
}

impl TryFrom<DeviceToSend> for Device {
    type Error = DeviceRepositoryError;

    fn try_from(device_to_send: DeviceToSend) -> Result<Self, Self::Error> {
        let device_id = Uuid::from_str(&device_to_send.id)
            .map_err(|e| DeviceRepositoryError::InternalError(e.to_string()))?;
        let user_id = Uuid::from_str(&device_to_send.user_id)
            .map_err(|e| DeviceRepositoryError::InternalError(e.to_string()))?;
        let name = device_to_send.name.to_string();
        let mut events = HashMap::new();
        for (key, val) in device_to_send.events.iter() {
            let event_format = EventFormat::try_from(val.format.as_str())
                .map_err(|e| DeviceRepositoryError::InternalError(e.to_string()))?;
            let mut event_payload = HashMap::new();
            for (event_key, event_value) in val.payload.clone() {
                let value = EventDataType::from_str(&event_value)
                    .map_err(|e| DeviceRepositoryError::InternalError(e))?;
                event_payload.insert(event_key, value);
            }

            events.insert(
                key.clone(),
                EventEmittable::new(event_format, event_payload),
            );
        }
        let mut actions = HashMap::new();
        for (key, val) in device_to_send.actions.iter() {
            let action_format = ActionFormat::try_from(val.format.as_str())
                .map_err(|e| DeviceRepositoryError::InternalError(e.to_string()))?;
            let mut action_payload = HashMap::new();
            for (action_key, action_value) in val.payload.clone() {
                let value = ActionDataType::from_str(&action_value)
                    .map_err(|e| DeviceRepositoryError::InternalError(e))?;
                action_payload.insert(action_key, value);
            }

            actions.insert(
                key.clone(),
                ActionEmittable::new(action_format, action_payload),
            );
        }
        Ok(Device::new(
            &device_id,
            &device_to_send.physical_id,
            &user_id,
            &name,
            events,
            actions,
        ))
    }
}

#[derive(Serialize, Deserialize)]
pub struct DeviceStateToSend {
    pub device_id: String,
    pub last_update: String,
    pub values: HashMap<String, Value>,
}

impl From<DeviceState> for DeviceStateToSend {
    fn from(device_state: DeviceState) -> Self {
        DeviceStateToSend {
            device_id: device_state.device_id.to_string(),
            last_update: device_state.last_update.to_rfc3339(),
            values: device_state
                .values
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect(),
        }
    }
}

impl TryFrom<DeviceStateToSend> for DeviceState {
    type Error = DeviceStateRepositoryError;

    fn try_from(device_state_to_send: DeviceStateToSend) -> Result<Self, Self::Error> {
        let device_id = Uuid::from_str(&device_state_to_send.device_id)
            .map_err(|e| DeviceStateRepositoryError::InternalError(e.to_string()))?;
        let last_update = chrono::DateTime::parse_from_rfc3339(&device_state_to_send.last_update)
            .map_err(|e| DeviceStateRepositoryError::InternalError(e.to_string()))?
            .with_timezone(&chrono::Utc);
        //let values = device_state_to_send.values.into_iter().map(|(k, v)| (k, v.try_into()));
        let mut values = HashMap::new();
        for (key, val) in device_state_to_send.values.into_iter() {
            let val = EventDataValue::try_from(val).map_err(|_| {
                DeviceStateRepositoryError::InternalError(format!("Unknown type given in {}", &key))
            })?;
            values.insert(key, val);
        }
        Ok(DeviceState {
            device_id,
            last_update,
            values,
        })
    }
}

#[derive(Serialize)]
pub struct EventToSend {
    pub id: String,
    pub device_id: String,
    pub timestamp: String,
    pub payload: String,
}

#[derive(Deserialize)]
pub struct EventToReceive {
    pub id: String,
    pub device_physical_id: String,
    pub event_name: String,
    pub timestamp: String,
    pub payload: HashMap<String, Value>,
}

impl TryFrom<EventToReceive> for Event {
    type Error = EventRepositoryError;

    fn try_from(event_to_send: EventToReceive) -> Result<Self, Self::Error> {
        let id = Uuid::from_str(&event_to_send.id)
            .map_err(|e| EventRepositoryError::RepositoryError(e.to_string()))?;
        let device_physical_id = Uuid::from_str(&event_to_send.device_physical_id)
            .map_err(|e| EventRepositoryError::RepositoryError(e.to_string()))?;
        let timestamp = chrono::DateTime::parse_from_rfc3339(&event_to_send.timestamp)
            .map_err(|e| EventRepositoryError::RepositoryError(e.to_string()))?
            .with_timezone(&chrono::Utc);
        let mut payload = HashMap::new();
        for (key, val) in event_to_send.payload.into_iter() {
            let val = EventDataValue::try_from(val).map_err(|_| {
                EventRepositoryError::RepositoryError(format!("Unknown type given in {}", &key))
            })?;
            payload.insert(key, val);
        }
        Ok(Event {
            id,
            device_physical_id: device_physical_id.to_string(),
            event_name: event_to_send.event_name,
            timestamp,
            payload,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventEmittableToSend {
    pub format: String,
    pub payload: HashMap<String, String>,
}

impl From<&EventEmittable> for EventEmittableToSend {
    fn from(value: &EventEmittable) -> Self {
        EventEmittableToSend {
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
pub struct ActionEmittableToSend {
    pub format: String,
    pub payload: HashMap<String, String>,
}

impl From<&ActionEmittable> for ActionEmittableToSend {
    fn from(value: &ActionEmittable) -> Self {
        ActionEmittableToSend {
            format: value.format().to_string(),
            payload: value
                .payload()
                .iter()
                .map(|(k, v)| (k.clone(), v.to_string()))
                .collect(),
        }
    }
}
