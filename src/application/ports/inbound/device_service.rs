use std::{collections::HashMap, fmt::Display};

use uuid::Uuid;

use crate::domain::{action::action_emittable::ActionEmittable, device::Device, event::event_emittable::EventEmittable};

pub enum DeviceServiceError {
    NotFound,
    AlreadyExists,
    InvalidInput,
    InternalError(String),
}

impl Display for DeviceServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeviceServiceError::NotFound => write!(f, "device not found"),
            DeviceServiceError::AlreadyExists => write!(f, "device already exists"),
            DeviceServiceError::InvalidInput => write!(f, "invalid input provided"),
            DeviceServiceError::InternalError(e) => write!(f, "internal error: {}", e),
        }
    }
}

pub trait DeviceService {
    async fn create_device(&self, device: &Device) -> Result<Device, DeviceServiceError>;
    async fn get_device(&self, id: Uuid) -> Result<Option<Device>, DeviceServiceError>;
    async fn get_devices_by_user_id(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<Device>, DeviceServiceError>;
    async fn get_device_by_physical_id(
        &self,
        physical_id: &str,
    ) -> Result<Option<Device>, DeviceServiceError>;
    async fn delete_device(&self, id: Uuid) -> Result<(), DeviceServiceError>;
    async fn update_device(
        &self,
        id: Uuid,
        physical_id: Option<String>,
        name: Option<String>,
        event: Option<HashMap<String, EventEmittable>>,
        actions: Option<HashMap<String, ActionEmittable>>
    ) -> Result<Device, DeviceServiceError>;
}
