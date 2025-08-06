use std::{str::FromStr, sync::{Arc, Mutex}};

use tracing::trace;
use uuid::Uuid;

use crate::{application::ports::{app::AppOutbound, inbound::device_service::DeviceService}, domain::device::Device, infrastructure::{ui::inbound::LoadingStatus, utils::log_device_service_error}};

#[derive(Debug)]
pub struct DeviceManager{
    user_id: Uuid,
    device_list: Arc<Mutex<Result<Vec<Device>, LoadingStatus>>>,
}

impl DeviceManager {
    pub fn new() -> Self {
        let user_id = Uuid::from_str("4a78a953-99bc-4a08-932e-956ef3f7d8fc").unwrap(); // Example user ID
        DeviceManager {
            user_id,
            device_list: Arc::new(Mutex::new(Err(LoadingStatus::NotStarted))),
        }
    }

    #[tracing::instrument]
    pub fn load_devices<'a>(&mut self, app_outbound: impl AppOutbound + 'static) {
        let device_list = self.device_list.clone();
        let user_id = self.user_id;
        // Lock the device list to set the status to InProgress
        *device_list.lock().unwrap() = Err(LoadingStatus::InProgress);

        tokio::spawn(async move {
            let device_service = app_outbound.get_device_service();
            match device_service.get_devices_by_user_id(user_id).await {
                Ok(devices) => {
                    trace!(result = "success");
                    *device_list.lock().unwrap() = Ok(devices);
                }
                Err(e) => {
                    log_device_service_error(&e);
                    *device_list.lock().unwrap() = Err(LoadingStatus::Failed("Failed to load devices".to_string()));
                }
            }
        });
    }

    pub fn get_device_list(&self) -> Arc<Mutex<Result<Vec<Device>, LoadingStatus>>> {
        self.device_list.clone()
    }
}