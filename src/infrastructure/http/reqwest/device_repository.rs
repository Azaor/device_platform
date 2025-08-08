use uuid::Uuid;

use crate::{
    application::ports::outbound::device_repository::{
        CreateDeviceRepository, DeleteDeviceRepository, DeviceRepositoryError, GetDeviceRepository,
        UpdateDeviceRepository,
    },
    domain::device::Device,
    infrastructure::http::reqwest::types::DeviceToSend,
};

#[derive(Debug)]
pub struct ReqwestDeviceRepository {
    base_url: String,
    create_path: String,
    get_path: String,
    get_by_physical_id_path: String,
    update_path: String,
    delete_path: String,
}

impl ReqwestDeviceRepository {
    pub fn new(
        base_url: &str,
        create_path: &str,
        get_path: &str,
        get_by_physical_id_path: &str,
        update_path: &str,
        delete_path: &str,
    ) -> Self {
        return ReqwestDeviceRepository {
            base_url: base_url.to_string(),
            create_path: create_path.to_string(),
            get_path: get_path.to_string(),
            get_by_physical_id_path: get_by_physical_id_path.to_string(),
            update_path: update_path.to_string(),
            delete_path: delete_path.to_string(),
        };
    }
}

impl CreateDeviceRepository for ReqwestDeviceRepository {
    async fn create(&self, device: &Device) -> Result<(), DeviceRepositoryError> {
        let device_to_send = DeviceToSend::from(device.clone());
        let client = reqwest::Client::new();
        let url = format!("{}{}", self.base_url, self.create_path);
        let res = client
            .post(&url)
            .json(&device_to_send)
            .header("Content-Type", "application/json")
            .send()
            .await
            .map_err(|e| DeviceRepositoryError::InternalError(e.to_string()))?;

        if res.status().is_success() {
            Ok(())
        } else {
            Err(DeviceRepositoryError::InternalError(
                res.status().to_string(),
            ))
        }
    }
}

impl GetDeviceRepository for ReqwestDeviceRepository {
    async fn get_by_id(&self, id: uuid::Uuid) -> Result<Option<Device>, DeviceRepositoryError> {
        let client = reqwest::Client::new();
        let url = format!("{}{}/{}", self.base_url, self.get_path, id.to_string());
        let res = client
            .get(&url)
            .send()
            .await
            .map_err(|e| DeviceRepositoryError::InternalError(e.to_string()))?;

        if res.status().is_success() {
            let device_to_send: DeviceToSend = res
                .json()
                .await
                .map_err(|e| DeviceRepositoryError::InternalError(e.to_string()))?;
            let device = Device::try_from(device_to_send)?;
            Ok(Some(device))
        } else {
            Err(DeviceRepositoryError::InternalError(
                res.status().to_string(),
            ))
        }
    }

    async fn get_by_user_id(
        &self,
        _user_id: uuid::Uuid,
    ) -> Result<Vec<Device>, DeviceRepositoryError> {
        let client = reqwest::Client::new();
        let url = format!("{}{}", self.base_url, self.get_path);
        let res = client
            .get(&url)
            .send()
            .await
            .map_err(|e| {
                println!("{}", e);
                DeviceRepositoryError::InternalError(e.to_string())
            })?;

        if res.status().is_success() {
            let devices_to_send: Vec<DeviceToSend> = res
                .json()
                .await
                .map_err(|e| DeviceRepositoryError::InternalError(e.to_string()))?;
            let mut devices = Vec::new();
            for device_to_send in devices_to_send {
                let device = Device::try_from(device_to_send)?;
                devices.push(device);
            }
            Ok(devices)
        } else {
            Err(DeviceRepositoryError::InternalError(
                res.status().to_string(),
            ))
        }
    }
    
    async fn get_by_physical_id(&self, physical_id: &str) -> Result<Option<Device>, DeviceRepositoryError> {
        let client = reqwest::Client::new();
        let url = format!("{}{}/{}", self.base_url, self.get_by_physical_id_path, physical_id.to_string());
        let res = client
            .get(&url)
            .send()
            .await
            .map_err(|e| DeviceRepositoryError::InternalError(e.to_string()))?;

        if res.status().is_success() {
            let device_to_send: DeviceToSend = res
                .json()
                .await
                .map_err(|e| DeviceRepositoryError::InternalError(e.to_string()))?;
            let device = Device::try_from(device_to_send)?;
            Ok(Some(device))
        } else {
            Err(DeviceRepositoryError::InternalError(
                res.status().to_string(),
            ))
        }
    }
}

impl UpdateDeviceRepository for ReqwestDeviceRepository {
    async fn update(&self, device: &Device) -> Result<(), DeviceRepositoryError> {
        let device_to_send = DeviceToSend::from(device.clone());
        let client = reqwest::Client::new();
        let url = format!(
            "{}{}/{}",
            self.base_url,
            self.update_path,
            device.id().to_string()
        );
        let res = client
            .put(&url)
            .json(&device_to_send)
            .header("Content-Type", "application/json")
            .send()
            .await
            .map_err(|e| DeviceRepositoryError::InternalError(e.to_string()))?;

        if res.status().is_success() {
            Ok(())
        } else {
            Err(DeviceRepositoryError::InternalError(
                res.status().to_string(),
            ))
        }
    }
}

impl DeleteDeviceRepository for ReqwestDeviceRepository {
    async fn delete_by_id(&self, id: Uuid) -> Result<(), DeviceRepositoryError> {
        let client = reqwest::Client::new();
        let url = format!("{}{}/{}", self.base_url, self.delete_path, id.to_string());
        let res = client
            .delete(&url)
            .send()
            .await
            .map_err(|e| DeviceRepositoryError::InternalError(e.to_string()))?;

        if res.status().is_success() {
            Ok(())
        } else {
            Err(DeviceRepositoryError::InternalError(
                res.status().to_string(),
            ))
        }
    }
}
