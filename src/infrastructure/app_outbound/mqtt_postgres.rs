use std::{env::VarError, sync::Arc};
use tokio::task;

use crate::{
    application::{
        ports::{
            app::AppOutbound, outbound::{
                device_repository::{
                    CreateDeviceRepository, DeleteDeviceRepository,
                    GetDeviceRepository, UpdateDeviceRepository,
                }, device_state_repository::{CreateDeviceStateRepository, DeleteDeviceStateRepository, GetDeviceStateRepository, UpdateDeviceStateRepository}, event_repository::{CreateEventRepository, GetEventRepository}
            }
        },
        usecases::{
            manage_device::ManageDeviceService, manage_device_state::ManageDeviceStateService,
            manage_event::ManageEventService,
        },
    },
    infrastructure::{
        utils, db::postgres::{
            device_repository::PostgresDeviceRepository, device_state_repository::PostgresDeviceStateRepository, event_repository::PostgresEventRepository
        }, mqtt::outbound::{
            device_repository::MqttDeviceRepository,
            device_state_repository::MqttDeviceStateRepository,
            event_repository::MqttEventRepository,
        }
    },
};

pub struct MqttAppOutbound {
    device_service: Arc<
        ManageDeviceService<
            MqttDeviceRepository,
            PostgresDeviceRepository,
            MqttDeviceRepository,
            MqttDeviceRepository,
        >,
    >,
    device_state_service: Arc<ManageDeviceStateService<MqttDeviceStateRepository, PostgresDeviceStateRepository, MqttDeviceStateRepository, MqttDeviceStateRepository>>,
    device_events_service: Arc<ManageEventService<MqttEventRepository, PostgresEventRepository>>,
}

impl MqttAppOutbound {
    pub async fn new() -> Result<Self, VarError> {
        // Load env files or environment variables
        dotenv::dotenv().ok();
        // Load configuration from environment variables or configuration files
        let mqtt_config = utils::load_mqtt_config_from_env()?;
        let (mqtt_client, mut event_loop) = utils::create_mqtt_client(&mqtt_config).await;
        let postgres_config = utils::load_postgres_config_from_env()?;
        let pool = utils::create_pool(postgres_config).await;

        let mqtt_device_repo = MqttDeviceRepository::new(mqtt_client.clone(), &mqtt_config.device_topic);
        let mqtt_device_state_repo = MqttDeviceStateRepository::new(mqtt_client.clone(), &mqtt_config.device_state_topic);
        let mqtt_event_repo = MqttEventRepository::new(mqtt_client, &mqtt_config.event_topic);

        let postgres_device_repo = PostgresDeviceRepository::new(pool.clone()).await;
        let postgres_device_state_repo = PostgresDeviceStateRepository::new(pool.clone()).await;
        let postgres_event_repo = PostgresEventRepository::new(pool.clone()).await;

        // Initialize the repositories
        postgres_device_repo.init().await;
        postgres_device_state_repo.init().await;
        postgres_event_repo.init().await;

        let arc_mqtt_device_repo = Arc::new(mqtt_device_repo);
        let arc_mqtt_event_repo = Arc::new(mqtt_event_repo);
        let arc_mqtt_device_state_repo = Arc::new(mqtt_device_state_repo);
        let arc_postgres_device_repo = Arc::new(postgres_device_repo);
        let arc_postgres_event_repo = Arc::new(postgres_event_repo);
        let arc_postgres_device_state_repo = Arc::new(postgres_device_state_repo);
        let device_service = Arc::new(ManageDeviceService {
            create_repo: arc_mqtt_device_repo.clone(),
            get_repo: arc_postgres_device_repo,
            update_repo: arc_mqtt_device_repo.clone(),
            delete_repo: arc_mqtt_device_repo,
        });
        let device_state_service = Arc::new(ManageDeviceStateService {
            create_repo: arc_mqtt_device_state_repo.clone(),
            get_repo: arc_postgres_device_state_repo,
            update_repo: arc_mqtt_device_state_repo.clone(),
            delete_repo: arc_mqtt_device_state_repo,
        });
        let device_events_service = Arc::new(ManageEventService {
            create_repo: arc_mqtt_event_repo,
            get_repo: arc_postgres_event_repo,
        });

        task::spawn(async move {
            while let Ok(_) = event_loop.poll().await {
                
            }
        });

        Ok(MqttAppOutbound {
            device_service,
            device_state_service,
            device_events_service,
        })
    }
}

impl AppOutbound for MqttAppOutbound {
    fn get_device_state_service(
        &self,
    ) -> &Arc<ManageDeviceStateService<impl CreateDeviceStateRepository, impl GetDeviceStateRepository, impl UpdateDeviceStateRepository, impl DeleteDeviceStateRepository>> {
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
