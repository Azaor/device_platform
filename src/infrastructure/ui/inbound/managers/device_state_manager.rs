use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::{
    application::ports::{
        app::AppOutbound,
        inbound::{device_service::DeviceService, device_state_service::DeviceStateService},
    }, domain::event::event_data_value::EventDataValue, infrastructure::ui::inbound::{egui_app::try_lock_until_success, LoadingStatus}
};

pub struct DeviceStateManager {
    device_states: Arc<Mutex<LoadingStatus<HashMap<Uuid, DisplayableDeviceState>>>>,
}

impl DeviceStateManager {
    pub fn new() -> Self {
        DeviceStateManager {
            device_states: Arc::new(Mutex::new(LoadingStatus::NotStarted)),
        }
    }

    pub fn load_device_state(
        &self,
        user_id: Uuid,
        app_outbound: impl AppOutbound + 'static,
    ) -> Result<(), String> {
        let mut device_states = try_lock_until_success(&self.device_states);
        match device_states.clone() {
            LoadingStatus::Success(val) => {
                *device_states = LoadingStatus::InProgress(Some(val));
                drop(device_states);
                self.fetch_data_in_other_thread(app_outbound, user_id);
            }
            LoadingStatus::NotStarted | LoadingStatus::Failed(_) => {
                drop(device_states); // Release the lock before spawning the task
                self.fetch_data_in_other_thread(app_outbound, user_id);
            }
            LoadingStatus::InProgress(_) => {}
        }
        return Ok(());
    }

    pub fn get_device_states(
        &self,
    ) -> Arc<Mutex<LoadingStatus<HashMap<Uuid, DisplayableDeviceState>>>> {
        self.device_states.clone()
    }

    fn fetch_data_in_other_thread(&self, app_outbound: impl AppOutbound + 'static, user_id: Uuid) {
        let device_states = self.device_states.clone();
        tokio::spawn(async move {
            let device_service = app_outbound.get_device_service();
            let devices = match device_service.get_devices_by_user_id(user_id).await {
                Ok(devices) => devices,
                Err(e) => {
                    let mut device_states = try_lock_until_success(&device_states);
                    *device_states = LoadingStatus::Failed(e.to_string());
                    return;
                }
            };
            let device_state_service = app_outbound.get_device_state_service();
            let mut device_states_list = HashMap::new();
            for device in devices {
                let device_state = match device_state_service.get_device_state(*device.id()).await {
                    Ok(s) => s,
                    Err(e) => {
                        let mut device_states = try_lock_until_success(&device_states);
                        *device_states = LoadingStatus::Failed(e.to_string());
                        return;
                    }
                };
                let state_to_insert = match device_state {
                    Some(state) => DisplayableDeviceState {
                        device_id: *device.id(),
                        device_name: device.name().to_string(),
                        values: Some(state.values.clone()),
                        last_update: Some(state.last_update),
                    },
                    None => DisplayableDeviceState {
                        device_id: *device.id(),
                        device_name: device.name().to_string(),
                        values: None,
                        last_update: None,
                    }, // Skip devices with no state
                };
                device_states_list.insert(*device.id(), state_to_insert);
            }
            let mut device_states = try_lock_until_success(&device_states);
            *device_states = LoadingStatus::Success(device_states_list);
            return;
        });
    }
}

#[derive(Debug, Clone)]
pub struct DisplayableDeviceState {
    pub device_id: Uuid,
    pub device_name: String,
    pub values: Option<HashMap<String, EventDataValue>>,
    pub last_update: Option<DateTime<Utc>>,
}
