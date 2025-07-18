use std::collections::HashMap;

use uuid::Uuid;

use crate::{application::ports::{inbound::device_state_service::{DeviceStateService, DeviceStateServiceError}, outbound::device_state_repository::{DeviceStateRepository, DeviceStateRepositoryError}}, domain::state::DeviceState};

pub struct ManageDeviceStateService<R: DeviceStateRepository> {
    pub repo: R
}

impl<R: DeviceStateRepository> DeviceStateService for ManageDeviceStateService<R> {
    async fn create_device_state(&self, device_id: Uuid, values: HashMap<String, String>) -> Result<DeviceState, DeviceStateServiceError> {
        let device_state = DeviceState {
            device_id,
            last_update: chrono::Utc::now(),
            values,
        };

        match self.repo.save(&device_state).await {
            Ok(_) => Ok(device_state),
            Err(DeviceStateRepositoryError::Conflict) => Err(DeviceStateServiceError::AlreadyExists),
            Err(DeviceStateRepositoryError::InternalError) => Err(DeviceStateServiceError::InternalError),
            Err(DeviceStateRepositoryError::DeviceNotFound) => Err(DeviceStateServiceError::DeviceNotFound),
        }
    }

    async fn get_device_state(&self, id: Uuid) -> Result<Option<DeviceState>, DeviceStateServiceError> {
        match self.repo.find_by_id(id).await {
            Ok(Some(device_state)) => Ok(Some(device_state)),
            Ok(None) => Err(DeviceStateServiceError::DeviceStateNotFound),
            Err(DeviceStateRepositoryError::DeviceNotFound) => Err(DeviceStateServiceError::DeviceNotFound),
            Err(DeviceStateRepositoryError::InternalError) => Err(DeviceStateServiceError::InternalError),
            Err(DeviceStateRepositoryError::Conflict) => Err(DeviceStateServiceError::AlreadyExists),
        }
    }
    
    async fn delete_device_state(&self, id: Uuid) -> Result<(), DeviceStateServiceError> {
        match self.repo.delete_by_id(id).await {
            Ok(_) => Ok(()),
            Err(DeviceStateRepositoryError::DeviceNotFound) => Err(DeviceStateServiceError::DeviceNotFound),
            Err(DeviceStateRepositoryError::InternalError) => Err(DeviceStateServiceError::InternalError),
            Err(DeviceStateRepositoryError::Conflict) => Err(DeviceStateServiceError::AlreadyExists),
        }
    }
    
    async fn update_device_state(&self, id: Uuid, values: HashMap<String, String>) -> Result<DeviceState, DeviceStateServiceError> {
        let mut device_state = match self.repo.find_by_id(id).await {
            Ok(Some(state)) => state,
            Ok(None) => return Err(DeviceStateServiceError::DeviceNotFound),
            Err(DeviceStateRepositoryError::DeviceNotFound) => return Err(DeviceStateServiceError::DeviceNotFound),
            Err(DeviceStateRepositoryError::InternalError) => return Err(DeviceStateServiceError::InternalError),
            Err(DeviceStateRepositoryError::Conflict) => return Err(DeviceStateServiceError::AlreadyExists),
        };

        device_state.values = values;
        device_state.last_update = chrono::Utc::now();

        match self.repo.save(&device_state).await {
            Ok(_) => Ok(device_state),
            Err(DeviceStateRepositoryError::Conflict) => Err(DeviceStateServiceError::AlreadyExists),
            Err(DeviceStateRepositoryError::InternalError) => Err(DeviceStateServiceError::InternalError),
            Err(DeviceStateRepositoryError::DeviceNotFound) => Err(DeviceStateServiceError::DeviceNotFound),
        }
    }
}