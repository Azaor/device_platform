use crate::{
    application::ports::outbound::device_state_repository::{
        CreateDeviceStateRepository, DeleteDeviceStateRepository, DeviceStateRepositoryError,
        GetDeviceStateRepository, UpdateDeviceStateRepository,
    },
    domain::state::DeviceState,
    infrastructure::{
        http::reqwest::types::DeviceStateToSend,
    },
};

pub struct ReqwestDeviceStateRepository {
    base_url: String,
    create_path: String,
    get_path: String,
    update_path: String,
    delete_path: String,
}

impl ReqwestDeviceStateRepository {
    pub fn new(
        base_url: &str,
        create_path: &str,
        get_path: &str,
        update_path: &str,
        delete_path: &str,
    ) -> Self {
        return ReqwestDeviceStateRepository {
            base_url: base_url.to_string(),
            create_path: create_path.to_string(),
            get_path: get_path.to_string(),
            update_path: update_path.to_string(),
            delete_path: delete_path.to_string(),
        };
    }
}

impl CreateDeviceStateRepository for ReqwestDeviceStateRepository {
    async fn create(&self, device: &DeviceState) -> Result<(), DeviceStateRepositoryError> {
        let device_state_to_send = DeviceStateToSend::from(device.clone());
        let url = format!("{}{}", self.base_url, self.create_path);
        let client = reqwest::Client::new();
        let response = client
            .post(&url)
            .json(&device_state_to_send)
            .send()
            .await
            .map_err(|e| DeviceStateRepositoryError::InternalError(e.to_string()))?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(DeviceStateRepositoryError::InternalError(
                response.status().to_string(),
            ))
        }
    }
}

impl UpdateDeviceStateRepository for ReqwestDeviceStateRepository {
    async fn update(&self, device: &DeviceState) -> Result<(), DeviceStateRepositoryError> {
        let device_state_to_send = DeviceStateToSend::from(device.clone());
        let url = format!("{}{}", self.base_url, self.update_path);
        let client = reqwest::Client::new();
        let response = client
            .put(&url)
            .json(&device_state_to_send)
            .send()
            .await
            .map_err(|e| DeviceStateRepositoryError::InternalError(e.to_string()))?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(DeviceStateRepositoryError::InternalError(
                response.status().to_string(),
            ))
        }
    }
}

impl GetDeviceStateRepository for ReqwestDeviceStateRepository {
    async fn get_by_id(
        &self,
        id: uuid::Uuid,
    ) -> Result<Option<DeviceState>, DeviceStateRepositoryError> {
        let url = format!("{}{}/{}", self.base_url, self.get_path, id);
        let client = reqwest::Client::new();
        let response = client
            .get(&url)
            .send()
            .await
            .map_err(|e| DeviceStateRepositoryError::InternalError(e.to_string()))?;

        match response.status() {
            reqwest::StatusCode::OK => {
                let device_state_to_send: DeviceStateToSend = response
                    .json()
                    .await
                    .map_err(|e| DeviceStateRepositoryError::InternalError(e.to_string()))?;
                let device_state = DeviceState::try_from(device_state_to_send)?;
                Ok(Some(device_state))
            }
            reqwest::StatusCode::NOT_FOUND => Ok(None),
            status => Err(DeviceStateRepositoryError::InternalError(
                status.to_string(),
            )),
        }
    }
}

impl DeleteDeviceStateRepository for ReqwestDeviceStateRepository {
    async fn delete_by_id(&self, id: uuid::Uuid) -> Result<(), DeviceStateRepositoryError> {
        let url = format!("{}{}/{}", self.base_url, self.delete_path, id);
        let client = reqwest::Client::new();
        let response = client
            .delete(&url)
            .send()
            .await
            .map_err(|e| DeviceStateRepositoryError::InternalError(e.to_string()))?;

        if response.status().is_success() {
            Ok(())
        } else {
            Err(DeviceStateRepositoryError::InternalError(
                response.status().to_string(),
            ))
        }
    }
}
