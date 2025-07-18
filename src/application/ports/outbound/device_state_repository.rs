use uuid::Uuid;

use crate::domain::state::DeviceState;

pub enum DeviceStateRepositoryError {
    DeviceNotFound,
    Conflict,
    InternalError
}

pub trait DeviceStateRepository: Send + Sync {
    fn save(&self, device: &DeviceState) -> impl Future<Output = Result<(), DeviceStateRepositoryError>> + Send;
    fn find_by_id(&self, id: Uuid) -> impl Future<Output = Result<Option<DeviceState>, DeviceStateRepositoryError>> + Send;
    fn delete_by_id(&self, id: Uuid) -> impl Future<Output = Result<(), DeviceStateRepositoryError>> + Send;
}