use std::{collections::HashMap, sync::Arc};

use uuid::Uuid;

use crate::{application::ports::{inbound::device_service::{DeviceService, DeviceServiceError}, outbound::device_repository::{CreateDeviceRepository, DeleteDeviceRepository, DeviceRepositoryError, GetDeviceRepository, UpdateDeviceRepository}}, domain::device::{Device, EventDataType, EventFormat}};

pub struct ManageDeviceService<C: CreateDeviceRepository, G: GetDeviceRepository, U: UpdateDeviceRepository, D: DeleteDeviceRepository> {
    pub create_repo: Arc<C>,
    pub get_repo: Arc<G>,
    pub update_repo: Arc<U>,
    pub delete_repo: Arc<D>,
}

impl<C: CreateDeviceRepository, G: GetDeviceRepository, U: UpdateDeviceRepository, D: DeleteDeviceRepository> DeviceService for ManageDeviceService<C, G, U, D> {
    async fn create_device(&self, user_id: Uuid, name: String, event_format: EventFormat) -> Result<Device, DeviceServiceError> {
        let device = Device::new(&Uuid::new_v4(), &user_id, &name, event_format, HashMap::new());
        match self.create_repo.create(&device).await {
            Ok(_) => Ok(device),
            Err(DeviceRepositoryError::Conflict) => Err(DeviceServiceError::AlreadyExists),
            Err(DeviceRepositoryError::NotFound) => Err(DeviceServiceError::InternalError),
            Err(DeviceRepositoryError::InternalError) => Err(DeviceServiceError::InternalError),
        }
    }
    
    async fn get_device(&self, id: Uuid) -> Result<Option<Device>, DeviceServiceError> {
        match self.get_repo.get_by_id(id).await {
            Ok(Some(device)) => Ok(Some(device)),
            Ok(None) => Err(DeviceServiceError::NotFound),
            Err(DeviceRepositoryError::NotFound) => Err(DeviceServiceError::NotFound),
            Err(DeviceRepositoryError::InternalError) => Err(DeviceServiceError::InternalError),
            Err(_) => Err(DeviceServiceError::InternalError), // Catch-all for any other errors
        }
    }

    async fn delete_device(&self, id: Uuid) -> Result<(), DeviceServiceError> {
        match self.delete_repo.delete_by_id(id).await {
            Ok(_) => Ok(()),
            Err(DeviceRepositoryError::NotFound) => Err(DeviceServiceError::NotFound),
            Err(DeviceRepositoryError::InternalError) => Err(DeviceServiceError::InternalError),
            Err(_) => Err(DeviceServiceError::InternalError), // Catch-all for any other errors
        }
    }
    async fn update_device(&self, id: Uuid, name: Option<String>, event_data_raw: Option<Vec<(String, String)>>) -> Result<Device, DeviceServiceError> {
        let mut device = match self.get_repo.get_by_id(id).await {
            Ok(Some(device)) => device,
            Ok(None) => return Err(DeviceServiceError::NotFound),
            Err(DeviceRepositoryError::NotFound) => return Err(DeviceServiceError::NotFound),
            Err(DeviceRepositoryError::InternalError) => return Err(DeviceServiceError::InternalError),
            Err(_) => return Err(DeviceServiceError::InternalError), // Catch-all for any other errors
        };

        if let Some(name) = name {
            device.name = name;
        }
        if let Some(event_data) = event_data_raw {
            let mut event_data_validated = HashMap::new();
            for (key, value) in event_data {
                let val = EventDataType::from_str(&value)
                    .map_err(|_| {
                        return DeviceServiceError::InvalidInput
                    })?;
                event_data_validated.insert(key, val);
            }
            device.event_data = event_data_validated;
        }

        match self.update_repo.update(&device).await {
            Ok(_) => Ok(device),
            Err(DeviceRepositoryError::Conflict) => Err(DeviceServiceError::AlreadyExists),
            Err(DeviceRepositoryError::NotFound) => Err(DeviceServiceError::InternalError),
            Err(DeviceRepositoryError::InternalError) => Err(DeviceServiceError::InternalError),
        }
    }
}