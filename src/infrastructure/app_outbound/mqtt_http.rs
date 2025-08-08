use std::{env::VarError, sync::Arc};
use tokio::task;

use crate::{
    application::{
        ports::{
            app::AppOutbound,
            outbound::{
                device_repository::{
                    CreateDeviceRepository, DeleteDeviceRepository, GetDeviceRepository,
                    UpdateDeviceRepository,
                },
                device_state_repository::{
                    CreateDeviceStateRepository, DeleteDeviceStateRepository,
                    GetDeviceStateRepository, UpdateDeviceStateRepository,
                },
                event_repository::{CreateEventRepository, GetEventRepository},
            },
        },
        usecases::{
            manage_device::ManageDeviceService, manage_device_state::ManageDeviceStateService,
            manage_event::ManageEventService,
        },
    },
    infrastructure::{
        http::reqwest::{
            device_repository::ReqwestDeviceRepository,
            device_state_repository::ReqwestDeviceStateRepository,
            event_repository::ReqwestEventRepository,
        },
        mqtt::outbound::{
            device_repository::MqttDeviceRepository,
            device_state_repository::MqttDeviceStateRepository,
            event_repository::MqttEventRepository,
        },
        utils,
    },
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
}

impl Clone for MqttHttpAppOutbound {
    fn clone(&self) -> Self {
        Self {
            device_service: Arc::clone(&self.device_service),
            device_state_service: Arc::clone(&self.device_state_service),
            device_events_service: Arc::clone(&self.device_events_service),
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
        let mqtt_event_repo = MqttEventRepository::new(mqtt_client, &mqtt_config.event_topic);

        let http_device_repo = ReqwestDeviceRepository::new(
            &http_config.base_url,
            &http_config.device_create_path.unwrap_or_default(),
            &http_config.device_get_path.unwrap_or_default(),
            &http_config.device_get_by_physical_id_path.unwrap_or_default(),
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

        task::spawn(async move { while let Ok(_) = event_loop.poll().await {} });

        Ok(MqttHttpAppOutbound {
            device_service,
            device_state_service,
            device_events_service,
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
}
