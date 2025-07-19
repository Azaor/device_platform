use std::{collections::HashMap, sync::Mutex};

use uuid::Uuid;

use crate::{application::ports::outbound::device_state_repository::{CreateDeviceStateRepository, DeleteDeviceStateRepository, DeviceStateRepositoryError, GetDeviceStateRepository, UpdateDeviceStateRepository}, domain::state::DeviceState};

pub struct InMemoryDeviceStateRepository {
    device_states: Mutex<HashMap<Uuid, DeviceState>>,
}

impl InMemoryDeviceStateRepository {
    pub fn new() -> Self {
        Self {
            device_states: Mutex::new(HashMap::new()),
        }
    }
}

impl CreateDeviceStateRepository for InMemoryDeviceStateRepository {
    async fn create(&self, device_state: &DeviceState) -> Result<(), DeviceStateRepositoryError> {
        let mut map = self.device_states.lock().unwrap();
        map.insert(device_state.device_id, device_state.clone());
        Ok(())
    }
}

impl GetDeviceStateRepository for InMemoryDeviceStateRepository {
    async fn get_by_id(&self, id: Uuid) -> Result<Option<DeviceState>, DeviceStateRepositoryError> {
        let map = self.device_states.lock().unwrap();
        match map.get(&id) {
            Some(state) => Ok(Some(state.clone())),
            None => Ok(None),
        }
    }
}

impl UpdateDeviceStateRepository for InMemoryDeviceStateRepository {
    async fn update(&self, device_state: &DeviceState) -> Result<(), DeviceStateRepositoryError> {
        let mut map = self.device_states.lock().unwrap();
        if map.contains_key(&device_state.device_id) {
            map.insert(device_state.device_id, device_state.clone());
            Ok(())
        } else {
            Err(DeviceStateRepositoryError::DeviceNotFound)
        }
    }
}

impl DeleteDeviceStateRepository for InMemoryDeviceStateRepository {
    async fn delete_by_id(&self, id: Uuid) -> Result<(), DeviceStateRepositoryError> {
        let mut map = self.device_states.lock().unwrap();
        if map.remove(&id).is_some() {
            Ok(())
        } else {
            Err(DeviceStateRepositoryError::DeviceNotFound)
        }
    }
}