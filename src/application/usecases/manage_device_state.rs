use std::{collections::HashMap, sync::Arc};

use uuid::Uuid;

use crate::{application::ports::{inbound::device_state_service::{DeviceStateService, DeviceStateServiceError}, outbound::device_state_repository::{CreateDeviceStateRepository, DeleteDeviceStateRepository, DeviceStateRepositoryError, GetDeviceStateRepository, UpdateDeviceStateRepository}}, domain::{event::event_data_value::EventDataValue, state::DeviceState}};

#[derive(Debug)]
pub struct ManageDeviceStateService<C: CreateDeviceStateRepository, G: GetDeviceStateRepository, U: UpdateDeviceStateRepository, D: DeleteDeviceStateRepository> {
    pub create_repo: Arc<C>,
    pub get_repo: Arc<G>,
    pub update_repo: Arc<U>,
    pub delete_repo: Arc<D>,
}

impl<C: CreateDeviceStateRepository, G: GetDeviceStateRepository, U: UpdateDeviceStateRepository, D: DeleteDeviceStateRepository> DeviceStateService for ManageDeviceStateService<C, G, U, D> {
    async fn create_device_state(&self, device_id: Uuid, values: HashMap<String, EventDataValue>) -> Result<DeviceState, DeviceStateServiceError> {
        let device_state = DeviceState {
            device_id,
            last_update: chrono::Utc::now(),
            values,
        };

        match self.create_repo.create(&device_state).await {
            Ok(_) => Ok(device_state),
            Err(DeviceStateRepositoryError::Conflict) => Err(DeviceStateServiceError::AlreadyExists),
            Err(DeviceStateRepositoryError::InternalError(s)) => Err(DeviceStateServiceError::InternalError(s)),
            Err(DeviceStateRepositoryError::DeviceNotFound) => Err(DeviceStateServiceError::DeviceNotFound),
        }
    }

    async fn get_device_state(&self, id: Uuid) -> Result<Option<DeviceState>, DeviceStateServiceError> {
        match self.get_repo.get_by_id(id).await {
            Ok(Some(device_state)) => Ok(Some(device_state)),
            Ok(None) => Err(DeviceStateServiceError::DeviceStateNotFound),
            Err(DeviceStateRepositoryError::DeviceNotFound) => Err(DeviceStateServiceError::DeviceNotFound),
            Err(DeviceStateRepositoryError::InternalError(s)) => Err(DeviceStateServiceError::InternalError(s)),
            Err(DeviceStateRepositoryError::Conflict) => Err(DeviceStateServiceError::AlreadyExists),
        }
    }
    
    async fn delete_device_state(&self, id: Uuid) -> Result<(), DeviceStateServiceError> {
        match self.delete_repo.delete_by_id(id).await {
            Ok(_) => Ok(()),
            Err(DeviceStateRepositoryError::DeviceNotFound) => Err(DeviceStateServiceError::DeviceNotFound),
            Err(DeviceStateRepositoryError::InternalError(s)) => Err(DeviceStateServiceError::InternalError(s)),
            Err(DeviceStateRepositoryError::Conflict) => Err(DeviceStateServiceError::AlreadyExists),
        }
    }
    
    async fn update_device_state(&self, id: Uuid, values: HashMap<String, EventDataValue>) -> Result<DeviceState, DeviceStateServiceError> {
        let mut device_state = match self.get_repo.get_by_id(id).await {
            Ok(Some(state)) => state,
            Ok(None) => return self.create_device_state(id, values).await,
            Err(DeviceStateRepositoryError::DeviceNotFound) => return Err(DeviceStateServiceError::DeviceNotFound),
            Err(DeviceStateRepositoryError::InternalError(s)) => return Err(DeviceStateServiceError::InternalError(s)),
            Err(DeviceStateRepositoryError::Conflict) => return Err(DeviceStateServiceError::AlreadyExists),
        };

        device_state.values.extend(values);
        device_state.last_update = chrono::Utc::now();

        match self.update_repo.update(&device_state).await {
            Ok(_) => Ok(device_state),
            Err(DeviceStateRepositoryError::Conflict) => Err(DeviceStateServiceError::AlreadyExists),
            Err(DeviceStateRepositoryError::InternalError(s)) => Err(DeviceStateServiceError::InternalError(s)),
            Err(DeviceStateRepositoryError::DeviceNotFound) => Err(DeviceStateServiceError::DeviceNotFound),
        }
    }
}