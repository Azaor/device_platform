use uuid::Uuid;

use crate::domain::device::{Device, EventFormat};

pub enum DeviceServiceError {
    NotFound,
    AlreadyExists,
    InvalidInput,
    InternalError,
}

pub trait DeviceService {
    async fn create_device(&self, user_id: Uuid, name: String, event_format: EventFormat) -> Result<Device, DeviceServiceError>;
    async fn get_device(&self, id: Uuid) -> Result<Option<Device>, DeviceServiceError>;
    async fn delete_device(&self, id: Uuid) -> Result<(), DeviceServiceError>;
    async fn update_device(&self, id: Uuid, name: Option<String>, metadata: Option<Vec<(String, String)>>) -> Result<Device, DeviceServiceError>;
}