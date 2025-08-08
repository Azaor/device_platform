use std::{ sync::Arc};

use uuid::Uuid;

use crate::{
    application::ports::{
        inbound::device_service::{DeviceService, DeviceServiceError},
        outbound::device_repository::{
            CreateDeviceRepository, DeleteDeviceRepository, DeviceRepositoryError,
            GetDeviceRepository, UpdateDeviceRepository,
        },
    },
    domain::{device::{Device, EventDataType}},
};

#[derive(Debug)]
pub struct ManageDeviceService<
    C: CreateDeviceRepository,
    G: GetDeviceRepository,
    U: UpdateDeviceRepository,
    D: DeleteDeviceRepository,
> {
    pub create_repo: Arc<C>,
    pub get_repo: Arc<G>,
    pub update_repo: Arc<U>,
    pub delete_repo: Arc<D>,
}

impl<
    C: CreateDeviceRepository,
    G: GetDeviceRepository,
    U: UpdateDeviceRepository,
    D: DeleteDeviceRepository,
> DeviceService for ManageDeviceService<C, G, U, D>
{
    async fn create_device(&self, device: &Device) -> Result<Device, DeviceServiceError> {
        match self.create_repo.create(&device).await {
            Ok(_) => Ok(device.clone()),
            Err(DeviceRepositoryError::Conflict) => Err(DeviceServiceError::AlreadyExists),
            Err(DeviceRepositoryError::NotFound) => Err(DeviceServiceError::InternalError(format!("Unexpected not found error while creating device"))),
            Err(DeviceRepositoryError::InternalError(v)) => Err(DeviceServiceError::InternalError(v)),
        }
    }

    async fn get_device(&self, id: Uuid) -> Result<Option<Device>, DeviceServiceError> {
        match self.get_repo.get_by_id(id).await {
            Ok(Some(device)) => Ok(Some(device)),
            Ok(None) => Err(DeviceServiceError::NotFound),
            Err(DeviceRepositoryError::NotFound) => Err(DeviceServiceError::NotFound),
            Err(DeviceRepositoryError::InternalError(v)) => Err(DeviceServiceError::InternalError(v)),
            Err(DeviceRepositoryError::Conflict) => Err(DeviceServiceError::InternalError(format!("Unexpected conflict error while getting device"))), // Catch-all for any other errors
        }
    }

    async fn get_devices_by_user_id(&self, user_id: Uuid) -> Result<Vec<Device>, DeviceServiceError> {
        match self.get_repo.get_by_user_id(user_id).await {
            Ok(devices) => Ok(devices),
            Err(DeviceRepositoryError::NotFound) => Err(DeviceServiceError::NotFound),
            Err(DeviceRepositoryError::InternalError(v)) => Err(DeviceServiceError::InternalError(v)),
            Err(DeviceRepositoryError::Conflict) => Err(DeviceServiceError::InternalError(format!("Unexpected conflict error while getting from user device"))), // Catch-all for any other errors
        }
    }

    async fn delete_device(&self, id: Uuid) -> Result<(), DeviceServiceError> {
        match self.delete_repo.delete_by_id(id).await {
            Ok(_) => Ok(()),
            Err(DeviceRepositoryError::NotFound) => Err(DeviceServiceError::NotFound),
            Err(DeviceRepositoryError::InternalError(v)) => Err(DeviceServiceError::InternalError(v)),
            Err(DeviceRepositoryError::Conflict) => Err(DeviceServiceError::InternalError(format!("Unexpected conflict error while deleting device"))), // Catch-all for any other errors
        }
    }

    async fn update_device(
        &self,
        id: Uuid,
        name: Option<String>,
        event_data_raw: Option<Vec<(String, EventDataType)>>,
    ) -> Result<Device, DeviceServiceError> {
        let mut device = match self.get_repo.get_by_id(id).await {
            Ok(Some(device)) => device,
            Ok(None) => return Err(DeviceServiceError::NotFound),
            Err(DeviceRepositoryError::NotFound) => return Err(DeviceServiceError::NotFound),
            Err(DeviceRepositoryError::InternalError(v)) => {
                return Err(DeviceServiceError::InternalError(v));
            }
            Err(DeviceRepositoryError::Conflict) => return Err(DeviceServiceError::InternalError(format!("Unexpected conflict error while getting device"))), // Catch-all for any other errors
        };

        if let Some(name) = name {
            device.set_name(&name);
        }
        if let Some(event_data) = event_data_raw {
            device.set_event_data(event_data.into_iter().collect());
        }

        match self.update_repo.update(&device).await {
            Ok(_) => Ok(device),
            Err(DeviceRepositoryError::Conflict) => Err(DeviceServiceError::AlreadyExists),
            Err(DeviceRepositoryError::NotFound) => Err(DeviceServiceError::InternalError(format!("Unexpected not found error while creating device"))),
            Err(DeviceRepositoryError::InternalError(v)) => Err(DeviceServiceError::InternalError(v)),
        }
    }
    
    async fn get_device_by_physical_id(&self, physical_id: &str) -> Result<Option<Device>, DeviceServiceError> {
        match self.get_repo.get_by_physical_id(physical_id).await {
            Ok(Some(device)) => Ok(Some(device)),
            Ok(None) => Err(DeviceServiceError::NotFound),
            Err(DeviceRepositoryError::NotFound) => Err(DeviceServiceError::NotFound),
            Err(DeviceRepositoryError::InternalError(v)) => Err(DeviceServiceError::InternalError(v)),
            Err(DeviceRepositoryError::Conflict) => Err(DeviceServiceError::InternalError(format!("Unexpected conflict error while getting device by physical ID"))), // Catch-all for any other errors
        }
    }
}
