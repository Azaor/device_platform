use std::{collections::HashMap, str::FromStr};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{application::ports::outbound::{device_repository::DeviceRepositoryError, device_state_repository::DeviceStateRepositoryError, event_repository::EventRepositoryError}, domain::{device::{Device, EventDataType, EventFormat}, event::Event, state::DeviceState}};


#[derive(Serialize, Deserialize)]
pub struct DeviceToSend {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub event_format: String,
    pub event_data: HashMap<String, String>,
}

impl From<Device> for DeviceToSend {
    fn from(device: Device) -> Self {
        let device_id = device.id.to_string();
        let user_id = device.user_id.to_string();
        let name = device.name.to_string();
        let event_format = device.event_format.to_string();
        let event_data = device
            .event_data
            .iter()
            .map(|(key, val)| return (key.clone(), val.to_string()))
            .collect();
        DeviceToSend {
            id: device_id,
            user_id,
            name,
            event_format,
            event_data,
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
        let event_format = EventFormat::try_from(device_to_send.event_format.as_str())
            .map_err(|e| DeviceRepositoryError::InternalError(e.to_string()))?;
        let mut event_data = HashMap::new();
        for (key, val) in device_to_send.event_data.iter() {
            let value = EventDataType::from_str(val)
                .map_err(|e| DeviceRepositoryError::InternalError(e))?;
            event_data.insert(key.clone(), value);
        }
        Ok(Device {
            id: device_id,
            user_id,
            name,
            event_format,
            event_data,
        })
    }
}

#[derive(Serialize, Deserialize)]
pub struct DeviceStateToSend {
    pub device_id: String,
    pub last_update: String,
    pub values: HashMap<String, String>,
}

impl From<DeviceState> for DeviceStateToSend {
    fn from(device_state: DeviceState) -> Self {
        DeviceStateToSend {
            device_id: device_state.device_id.to_string(),
            last_update: device_state.last_update.to_rfc3339(),
            values: device_state
                .values
                .iter()
                .map(|(k, v)| (k.clone(), v.to_string()))
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
        let values = device_state_to_send.values.clone();
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
    pub device_id: String,
    pub timestamp: String,
    pub payload: HashMap<String, String>,
}

impl TryFrom<EventToReceive> for Event {
    type Error = EventRepositoryError;

    fn try_from(event_to_send: EventToReceive) -> Result<Self, Self::Error> {
        let id = Uuid::from_str(&event_to_send.id)
            .map_err(|e| EventRepositoryError::RepositoryError(e.to_string()))?;
        let device_id = Uuid::from_str(&event_to_send.device_id)
            .map_err(|e| EventRepositoryError::RepositoryError(e.to_string()))?;
        let timestamp = chrono::DateTime::parse_from_rfc3339(&event_to_send.timestamp)
            .map_err(|e| EventRepositoryError::RepositoryError(e.to_string()))?
            .with_timezone(&chrono::Utc);
        let payload = event_to_send.payload;
        Ok(Event {
            id,
            device_id,
            timestamp,
            payload,
        })
    }
}