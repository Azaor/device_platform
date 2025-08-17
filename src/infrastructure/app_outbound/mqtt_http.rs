use std::{collections::HashMap, env::VarError, str::FromStr, sync::Arc};
use chrono::DateTime;
use rumqttc::Packet;
use serde_json::Value;
use tokio::sync::Mutex;
use tracing::error;

use crate::{
    application::{
        ports::{
            app::AppOutbound,
            outbound::{
                action_repository::{ActionRepositoryError, HandleActionRepository}, device_repository::{
                    CreateDeviceRepository, DeleteDeviceRepository, GetDeviceRepository,
                    UpdateDeviceRepository,
                }, device_state_repository::{
                    CreateDeviceStateRepository, DeleteDeviceStateRepository,
                    GetDeviceStateRepository, UpdateDeviceStateRepository,
                }, event_repository::{CreateEventRepository, GetEventRepository}
            },
        },
        usecases::{
            manage_action::ManageActionService, manage_device::ManageDeviceService,
            manage_device_state::ManageDeviceStateService, manage_event::ManageEventService,
        },
    }, domain::action::{action::Action, action_format::ActionFormat}, infrastructure::{
        http::reqwest::{
            device_repository::ReqwestDeviceRepository,
            device_state_repository::ReqwestDeviceStateRepository,
            event_repository::ReqwestEventRepository,
        },
        mqtt::{mqtt_messages::{CreateActionPayload, MqttActionType, MqttMessage}, outbound::{
            action_repository::MqttActionRepository, device_repository::MqttDeviceRepository,
            device_state_repository::MqttDeviceStateRepository,
            event_repository::MqttEventRepository,
        }},
        utils::{self},
    }
};

#[derive(Debug)]
pub struct MqttHttpAppOutbound {
    device_service: Arc<
        ManageDeviceService<
            MqttDeviceRepository,
            ReqwestDeviceRepository,
            MqttDeviceRepository,
            MqttDeviceRepository,
        >,
    >,
    device_state_service: Arc<
        ManageDeviceStateService<
            MqttDeviceStateRepository,
            ReqwestDeviceStateRepository,
            MqttDeviceStateRepository,
            MqttDeviceStateRepository,
        >,
    >,
    device_events_service: Arc<ManageEventService<MqttEventRepository, ReqwestEventRepository>>,
    device_actions_service: Arc<ManageActionService<MqttActionRepository, LocalActionRepository>>,
}

impl Clone for MqttHttpAppOutbound {
    fn clone(&self) -> Self {
        Self {
            device_service: Arc::clone(&self.device_service),
            device_state_service: Arc::clone(&self.device_state_service),
            device_events_service: Arc::clone(&self.device_events_service),
            device_actions_service: Arc::clone(&self.device_actions_service),
        }
    }
}

impl MqttHttpAppOutbound {
    pub async fn new() -> Result<Self, VarError> {
        // Load env files or environment variables
        dotenv::dotenv().ok();
        // Load configuration from environment variables or configuration files
        let mqtt_config = utils::load_mqtt_config_from_env()?;
        let (mqtt_client, mut event_loop) = utils::create_mqtt_client(&mqtt_config).await;
        let http_config = utils::load_http_config_from_env()?;

        let mqtt_device_repo =
            MqttDeviceRepository::new(mqtt_client.clone(), &mqtt_config.device_topic);
        let mqtt_device_state_repo =
            MqttDeviceStateRepository::new(mqtt_client.clone(), &mqtt_config.device_state_topic);
        let mqtt_event_repo =
            MqttEventRepository::new(mqtt_client.clone(), &mqtt_config.event_topic);
        let mqtt_action_repo =
            MqttActionRepository::new(mqtt_client, &mqtt_config.action_topic);
        let local_action_repo = LocalActionRepository::new();

        let http_device_repo = ReqwestDeviceRepository::new(
            &http_config.base_url,
            &http_config.device_create_path.unwrap_or_default(),
            &http_config.device_get_path.unwrap_or_default(),
            &http_config
                .device_get_by_physical_id_path
                .unwrap_or_default(),
            &http_config.device_update_path.unwrap_or_default(),
            &http_config.device_delete_path.unwrap_or_default(),
        );
        let http_device_state_repo = ReqwestDeviceStateRepository::new(
            &http_config.base_url,
            &http_config.device_state_create_path.unwrap_or_default(),
            &http_config.device_state_get_path.unwrap_or_default(),
            &http_config.device_state_update_path.unwrap_or_default(),
            &http_config.device_state_delete_path.unwrap_or_default(),
        );
        let http_event_repo = ReqwestEventRepository::new(
            &http_config.base_url,
            &http_config.event_create_path.unwrap_or_default(),
            &http_config.event_get_path.unwrap_or_default(),
        );

        let arc_mqtt_device_repo = Arc::new(mqtt_device_repo);
        let arc_mqtt_event_repo = Arc::new(mqtt_event_repo);
        let arc_mqtt_device_state_repo = Arc::new(mqtt_device_state_repo);
        let arc_mqtt_action_repo = Arc::new(Mutex::new(mqtt_action_repo));
        let arc_local_action_repo = Arc::new(Mutex::new(local_action_repo));
        let arc_http_device_repo = Arc::new(http_device_repo);
        let arc_http_event_repo = Arc::new(http_event_repo);
        let arc_http_device_state_repo = Arc::new(http_device_state_repo);
        let device_service = Arc::new(ManageDeviceService {
            create_repo: arc_mqtt_device_repo.clone(),
            get_repo: arc_http_device_repo,
            update_repo: arc_mqtt_device_repo.clone(),
            delete_repo: arc_mqtt_device_repo,
        });
        let device_state_service = Arc::new(ManageDeviceStateService {
            create_repo: arc_mqtt_device_state_repo.clone(),
            get_repo: arc_http_device_state_repo,
            update_repo: arc_mqtt_device_state_repo.clone(),
            delete_repo: arc_mqtt_device_state_repo,
        });
        let device_events_service = Arc::new(ManageEventService {
            create_repo: arc_mqtt_event_repo,
            get_repo: arc_http_event_repo,
        });
        let device_actions_service = Arc::new(ManageActionService {
            create_repo: arc_mqtt_action_repo,
            get_repo: arc_local_action_repo.clone(),
        });
        let action_topic_cloned = mqtt_config.action_topic.clone();
        let device_id_cloned = "abc".to_string();
        tokio::task::spawn(async move {
            while let Ok(notification) = event_loop.poll().await {
                if let rumqttc::Event::Incoming(Packet::Publish(published)) = notification {
                    if published.topic == format!("{}/{}", action_topic_cloned, device_id_cloned) {
                        let data: MqttMessage<Value> =
                            match serde_json::from_slice(&published.payload) {
                                Ok(p) => p,
                                Err(e) => {
                                    error!(
                                        result = "error",
                                        details = format!("Invalid payload: {}", e.to_string())
                                    );
                                    continue;
                                }
                            };
                        match data.action_type {
                            MqttActionType::Create => {
                                let payload: CreateActionPayload =
                                    match serde_json::from_slice(&published.payload) {
                                        Ok(p) => p,
                                        Err(e) => {
                                            error!(
                                                result = "error",
                                                details =
                                                    format!("Invalid payload: {}", e.to_string())
                                            );
                                            continue;
                                        }
                                    };
                                let timestamp = match DateTime::from_str(&payload.timestamp) {
                                    Ok(t) => t,
                                    Err(_) => {
                                        error!(
                                            result = "error",
                                            details = "invalid timestamp format".to_string()
                                        );
                                        continue;
                                    }
                                };
                                let action_data = match ActionFormat::Json
                                    .decode_action(payload.action_data.as_bytes())
                                {
                                    Ok(a) => a,
                                    Err(_) => {
                                        error!(
                                            result = "error",
                                            details = "invalid timestamp format".to_string()
                                        );
                                        continue;
                                    }
                                };
                                let action = Action::new(
                                    payload.device_id.clone(),
                                    &payload.action_data,
                                    &timestamp,
                                    action_data,
                                );
                                let mut local_action_repo = arc_local_action_repo.lock().await;
                                local_action_repo.add_pending_action(&payload.device_id, action).await;
                            }
                            MqttActionType::Delete | MqttActionType::Update => {}
                        }
                    }
                }
            }
        });

        Ok(MqttHttpAppOutbound {
            device_service,
            device_state_service,
            device_events_service,
            device_actions_service,
        })
    }
}

impl AppOutbound for MqttHttpAppOutbound {
    fn get_device_state_service(
        &self,
    ) -> &Arc<
        ManageDeviceStateService<
            impl CreateDeviceStateRepository,
            impl GetDeviceStateRepository,
            impl UpdateDeviceStateRepository,
            impl DeleteDeviceStateRepository,
        >,
    > {
        &self.device_state_service
    }

    fn get_event_service(
        &self,
    ) -> &Arc<ManageEventService<impl CreateEventRepository, impl GetEventRepository>> {
        &self.device_events_service
    }

    fn get_device_service(
        &self,
    ) -> &Arc<
        ManageDeviceService<
            impl CreateDeviceRepository,
            impl GetDeviceRepository,
            impl UpdateDeviceRepository,
            impl DeleteDeviceRepository,
        >,
    > {
        &self.device_service
    }

    fn get_action_service(
        &self,
    ) -> &Arc<
        crate::application::usecases::manage_action::ManageActionService<
            impl crate::application::ports::outbound::action_repository::CreateActionRepository,
            impl crate::application::ports::outbound::action_repository::HandleActionRepository,
        >,
    > {
        &self.device_actions_service
    }
}

#[derive(Debug)]
struct LocalActionRepository {
    pending_actions: HashMap<String, Vec<Action>>
}

impl LocalActionRepository {
    fn new() -> Self {
        Self {
            pending_actions: HashMap::new()
        }
    }
    async fn add_pending_action(&mut self, device_id: &str, action: Action) {
        self.pending_actions
            .entry(device_id.to_string())
            .or_insert_with(Vec::new)
            .push(action);

    }
}

impl HandleActionRepository for LocalActionRepository {
    async fn get_actions(
        &mut self,
        device_id: &str,
    ) -> Result<Vec<Action>, ActionRepositoryError> {
        let actions = self.pending_actions.remove(device_id).unwrap_or_default();
        Ok(actions)
    }
}