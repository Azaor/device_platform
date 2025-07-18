use std::{collections::HashMap, sync::Mutex};

use uuid::Uuid;

use crate::{application::ports::outbound::device_state_repository::{DeviceStateRepository, DeviceStateRepositoryError}, domain::state::DeviceState};

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

impl DeviceStateRepository for InMemoryDeviceStateRepository {
    async fn save(&self, device_state: &DeviceState) -> Result<(), DeviceStateRepositoryError> {
        let mut map = self.device_states.lock().unwrap();
        map.insert(device_state.device_id, device_state.clone());
        Ok(())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<DeviceState>, DeviceStateRepositoryError> {
        let map = self.device_states.lock().unwrap();
        match map.get(&id) {
            Some(state) => Ok(Some(state.clone())),
            None => Ok(None),
        }
    }
    
    async fn delete_by_id(&self, id: Uuid) -> Result<(), DeviceStateRepositoryError> {
        let mut map = self.device_states.lock().unwrap();
        if map.remove(&id).is_some() {
            Ok(())
        } else {
            Err(DeviceStateRepositoryError::DeviceNotFound)
        }
    }
}