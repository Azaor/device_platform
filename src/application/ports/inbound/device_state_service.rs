use std::collections::HashMap;

use uuid::Uuid;

use crate::domain::{event::EventDataValue, state::DeviceState};

pub enum DeviceStateServiceError {
    DeviceNotFound,
    DeviceStateNotFound,
    AlreadyExists,
    InvalidInput,
    InternalError(String),
}

impl std::fmt::Display for DeviceStateServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeviceStateServiceError::DeviceNotFound => write!(f, "Device not found"),
            DeviceStateServiceError::DeviceStateNotFound => write!(f, "Device state not found"),
            DeviceStateServiceError::AlreadyExists => write!(f, "Device state already exists"),
            DeviceStateServiceError::InvalidInput => write!(f, "Invalid input provided"),
            DeviceStateServiceError::InternalError(s) => write!(f, "Internal error: {}", s),
        }
    }
}

pub trait DeviceStateService {
    async fn create_device_state(&self, device_id: Uuid, values: HashMap<String, EventDataValue>) -> Result<DeviceState, DeviceStateServiceError>;
    async fn get_device_state(&self, id: Uuid) -> Result<Option<DeviceState>, DeviceStateServiceError>;
    async fn delete_device_state(&self, id: Uuid) -> Result<(), DeviceStateServiceError>;
    async fn update_device_state(&self, id: Uuid, values: HashMap<String, EventDataValue>) -> Result<DeviceState, DeviceStateServiceError>;
}