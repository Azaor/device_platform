use uuid::Uuid;

use crate::domain::device::Device;

#[derive(Debug, Clone)]
pub enum DeviceRepositoryError {
    NotFound,
    Conflict,
    InternalError(String),
}

pub trait DeviceRepository: Send + Sync {
    fn save(
        &self,
        device: &Device,
    ) -> impl Future<Output = Result<(), DeviceRepositoryError>> + Send;
    fn find_by_id(
        &self,
        id: Uuid,
    ) -> impl Future<Output = Result<Option<Device>, DeviceRepositoryError>> + Send;
    fn delete_by_id(
        &self,
        id: Uuid,
    ) -> impl Future<Output = Result<(), DeviceRepositoryError>> + Send;
}

pub trait GetDeviceRepository: Send + Sync {
    fn get_by_id(
        &self,
        id: Uuid,
    ) -> impl Future<Output = Result<Option<Device>, DeviceRepositoryError>> + Send;
    fn get_by_user_id(
        &self,
        user_id: Uuid,
    ) -> impl Future<Output = Result<Vec<Device>, DeviceRepositoryError>> + Send;
    fn get_by_physical_id(
        &self,
        physical_id: &str,
    ) -> impl Future<Output = Result<Option<Device>, DeviceRepositoryError>> + Send;
}

pub trait CreateDeviceRepository: Send + Sync {
    fn create(
        &self,
        device: &Device,
    ) -> impl Future<Output = Result<(), DeviceRepositoryError>> + Send;
}

pub trait DeleteDeviceRepository: Send + Sync {
    fn delete_by_id(
        &self,
        id: Uuid,
    ) -> impl Future<Output = Result<(), DeviceRepositoryError>> + Send;
}

pub trait UpdateDeviceRepository: Send + Sync {
    fn update(
        &self,
        device: &Device,
    ) -> impl Future<Output = Result<(), DeviceRepositoryError>> + Send;
}
