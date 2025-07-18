use std::collections::HashMap;
use std::sync::Mutex;
use crate::application::ports::outbound::device_repository::{DeviceRepository, DeviceRepositoryError};
use crate::domain::device::Device;
use uuid::Uuid;

pub struct InMemoryDeviceRepository {
    pub store: Mutex<HashMap<Uuid, Device>>,
}

impl InMemoryDeviceRepository {
    pub fn new() -> Self {
        Self { store: Mutex::new(HashMap::new()) }
    }
}

impl DeviceRepository for InMemoryDeviceRepository {
    async fn save(&self, device: &Device) -> Result<(), DeviceRepositoryError> {
        let mut map = self.store.lock().unwrap();
        map.insert(device.id, device.clone());
        Ok(())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Device>, DeviceRepositoryError> {
        let map = self.store.lock().unwrap();
        match map.get(&id) {
            Some(device) => Ok(Some(device.clone())),
            None => Err(DeviceRepositoryError::NotFound),
        }
    }
    async fn delete_by_id(&self, id: Uuid) -> Result<(), DeviceRepositoryError> {
        let mut map = self.store.lock().unwrap();
        if map.remove(&id).is_some() {
            Ok(())
        } else {
            Err(DeviceRepositoryError::NotFound)
        }
    }
}
