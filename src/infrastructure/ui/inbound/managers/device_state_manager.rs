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
    },
    domain::{event::EventDataValue},
    infrastructure::ui::inbound::LoadingStatus,
};

pub struct DeviceStateManager {
    device_states: Arc<Mutex<Result<HashMap<Uuid, DisplayableDeviceState>, LoadingStatus>>>,
}

impl DeviceStateManager {
    pub fn new() -> Self {
        DeviceStateManager {
            device_states: Arc::new(Mutex::new(Err(LoadingStatus::NotStarted))),
        }
    }

    pub fn load_device_state(
        &self,
        user_id: Uuid,
        app_outbound: impl AppOutbound + 'static,
    ) -> Result<(), String> {
        let mut device_states = self.device_states.lock().map_err(|e| e.to_string())?;
        match *device_states {
            Ok(_) | Err(LoadingStatus::NotStarted) | Err(LoadingStatus::Failed(_)) => {
                *device_states = Err(LoadingStatus::InProgress);
                drop(device_states);
                let user_id_clone = user_id;
                let device_states_clone = self.device_states.clone();
                tokio::spawn(async move {
                    let device_service = app_outbound.get_device_service();
                    let devices = match device_service.get_devices_by_user_id(user_id_clone).await {
                        Ok(devices) => devices,
                        Err(e) => {
                            let mut locked_success = false;
                            while !locked_success {
                                match device_states_clone.lock() {
                                    Ok(mut device_states) => {
                                        *device_states = Err(LoadingStatus::Failed(e.to_string()));
                                        locked_success = true;
                                    },
                                    Err(_) => {
                                        device_states_clone.clear_poison();
                                    },
                                }
                            }
                            return;
                        }
                    };
                    let device_state_service = app_outbound.get_device_state_service();
                    let mut device_states_list = HashMap::new();
                    for device in devices {
                        let device_state = match device_state_service.get_device_state(device.id).await
                        {
                            Ok(s) => s,
                            Err(e) => {
                                let mut locked_success = false;
                                while !locked_success {
                                    match device_states_clone.lock() {
                                        Ok(mut device_states) => {
                                            *device_states = Err(LoadingStatus::Failed(e.to_string()));
                                            locked_success = true;
                                        },
                                        Err(_) => {
                                            device_states_clone.clear_poison();
                                        },
                                    }
                                }
                                return;
                            }
                        };
                        let state_to_insert = match device_state {
                            Some(state) => {
                                DisplayableDeviceState {
                                    device_id: device.id,
                                    device_name: device.name.clone(),
                                    values: Some(state.values.clone()),
                                    last_update: Some(state.last_update),
                                }
                            }
                            None => {
                                DisplayableDeviceState {
                                    device_id: device.id,
                                    device_name: device.name.clone(),
                                    values: None,
                                    last_update: None,
                                }
                            }, // Skip devices with no state
                        };
                        device_states_list.insert(device.id, state_to_insert);
                    }
                    let locked_success = false;
                    while !locked_success {
                        match device_states_clone.lock() {
                            Ok(mut device_states) => {
                                *device_states = Ok(device_states_list);
                                break;
                            },
                            Err(_) => {
                                device_states_clone.clear_poison();
                            },
                        }
                    }
                });
            },
            Err(LoadingStatus::InProgress) => ()
        }
        return Ok(());
    }

    pub fn get_device_states(&self) -> Arc<Mutex<Result<HashMap<Uuid, DisplayableDeviceState>, LoadingStatus>>> {
        self.device_states.clone()
    }
}

#[derive(Debug, Clone)]
pub struct DisplayableDeviceState {
    pub device_id: Uuid,
    pub device_name: String,
    pub values: Option<HashMap<String, EventDataValue>>,
    pub last_update: Option<DateTime<Utc>>,
}
