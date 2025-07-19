use uuid::Uuid;

use crate::domain::state::DeviceState;

pub enum DeviceStateRepositoryError {
    DeviceNotFound,
    Conflict,
    InternalError
}

pub trait GetDeviceStateRepository: Send + Sync {
    fn get_by_id(&self, id: Uuid) -> impl Future<Output = Result<Option<DeviceState>, DeviceStateRepositoryError>> + Send;
}

pub trait CreateDeviceStateRepository: Send + Sync {
    fn create(&self, device: &DeviceState) -> impl Future<Output = Result<(), DeviceStateRepositoryError>> + Send;
}

pub trait DeleteDeviceStateRepository: Send + Sync {
    fn delete_by_id(&self, id: Uuid) -> impl Future<Output = Result<(), DeviceStateRepositoryError>> + Send;
}

pub trait UpdateDeviceStateRepository: Send + Sync {
    fn update(&self, device: &DeviceState) -> impl Future<Output = Result<(), DeviceStateRepositoryError>> + Send;
}