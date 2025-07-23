use std::collections::HashMap;
use std::sync::Mutex;
use crate::application::ports::outbound::device_repository::{CreateDeviceRepository, DeleteDeviceRepository, DeviceRepositoryError, GetDeviceRepository, UpdateDeviceRepository};
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

impl CreateDeviceRepository for InMemoryDeviceRepository{
    async fn create(&self, device: &Device) -> Result<(), DeviceRepositoryError> {
        let mut map = self.store.lock().unwrap();
        map.insert(device.id, device.clone());
        Ok(())
    }
}

impl GetDeviceRepository for InMemoryDeviceRepository {
    async fn get_by_id(&self, id: Uuid) -> Result<Option<Device>, DeviceRepositoryError> {
        let map = self.store.lock().unwrap();
        match map.get(&id) {
            Some(device) => Ok(Some(device.clone())),
            None => Err(DeviceRepositoryError::NotFound),
        }
    }
    
    async fn get_by_user_id(&self, user_id: Uuid) -> Result<Vec<Device>, DeviceRepositoryError> {
        let map = self.store.lock().unwrap();
        let devices: Vec<Device> = map.values()
            .filter(|device| device.user_id == user_id)
            .cloned()
            .collect();
        Ok(devices)
    }
}

impl UpdateDeviceRepository for InMemoryDeviceRepository {
    async fn update(&self, device: &Device) -> Result<(), DeviceRepositoryError> {
        let mut map = self.store.lock().unwrap();
        if map.contains_key(&device.id) {
            map.insert(device.id, device.clone());
            Ok(())
        } else {
            Err(DeviceRepositoryError::NotFound)
        }
    }
}

impl DeleteDeviceRepository for InMemoryDeviceRepository {
    async fn delete_by_id(&self, id: Uuid) -> Result<(), DeviceRepositoryError> {
        let mut map = self.store.lock().unwrap();
        if map.remove(&id).is_some() {
            Ok(())
        } else {
            Err(DeviceRepositoryError::NotFound)
        }
    }
}