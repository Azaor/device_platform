use std::collections::HashMap;

use uuid::Uuid;

use crate::domain::{event::EventDataValue, state::DeviceState};

pub enum DeviceStateServiceError {
    DeviceNotFound,
    DeviceStateNotFound,
    AlreadyExists,
    InvalidInput,
    InternalError,
}

pub trait DeviceStateService {
    async fn create_device_state(&self, device_id: Uuid, values: HashMap<String, EventDataValue>) -> Result<DeviceState, DeviceStateServiceError>;
    async fn get_device_state(&self, id: Uuid) -> Result<Option<DeviceState>, DeviceStateServiceError>;
    async fn delete_device_state(&self, id: Uuid) -> Result<(), DeviceStateServiceError>;
    async fn update_device_state(&self, id: Uuid, values: HashMap<String, EventDataValue>) -> Result<DeviceState, DeviceStateServiceError>;
}