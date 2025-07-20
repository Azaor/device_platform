use std::{env::VarError, sync::Arc};

use crate::{
    application::{
        ports::{
            app::AppOutbound,
            outbound::{
                device_repository::{
                    CreateDeviceRepository, DeleteDeviceRepository, GetDeviceRepository,
                    UpdateDeviceRepository,
                },
                device_state_repository::{CreateDeviceStateRepository, DeleteDeviceStateRepository, GetDeviceStateRepository, UpdateDeviceStateRepository},
                event_repository::{CreateEventRepository, GetEventRepository},
            },
        },
        usecases::{
            manage_device::ManageDeviceService, manage_device_state::ManageDeviceStateService,
            manage_event::ManageEventService,
        },
    },
    infrastructure::{db::postgres::{
        device_repository::PostgresDeviceRepository,
        device_state_repository::PostgresDeviceStateRepository,
        event_repository::PostgresEventRepository,
    }, utils},
};

pub struct FullPostgresAppOutbound {
    device_service: Arc<
        ManageDeviceService<
            PostgresDeviceRepository,
            PostgresDeviceRepository,
            PostgresDeviceRepository,
            PostgresDeviceRepository,
        >,
    >,
    device_state_service: Arc<ManageDeviceStateService<PostgresDeviceStateRepository, PostgresDeviceStateRepository, PostgresDeviceStateRepository, PostgresDeviceStateRepository>>,
    device_events_service:
        Arc<ManageEventService<PostgresEventRepository, PostgresEventRepository>>,
}

impl FullPostgresAppOutbound {
    pub async fn new() -> Result<Self, VarError> {
        // Load env files or environment variables
        dotenv::dotenv().ok();
        // Load configuration from environment variables or configuration files
        let postgres_config = utils::load_postgres_config_from_env()?;
        let pool = utils::create_pool(postgres_config).await;
        let device_repo = PostgresDeviceRepository::new(pool.clone()).await;
        let device_state_repo = PostgresDeviceStateRepository::new(pool.clone()).await;
        let event_repo = PostgresEventRepository::new(pool.clone()).await;

        // Initialize the repositories
        device_repo.init().await;
        device_state_repo.init().await;
        event_repo.init().await;

        let arc_device_repo = Arc::new(device_repo);
        let arc_event_repo = Arc::new(event_repo);
        let arc_device_state_repo = Arc::new(device_state_repo);
        let device_service = Arc::new(ManageDeviceService {
            create_repo: arc_device_repo.clone(),
            get_repo: arc_device_repo.clone(),
            update_repo: arc_device_repo.clone(),
            delete_repo: arc_device_repo,
        });
        let device_state_service = Arc::new(ManageDeviceStateService {
            create_repo: arc_device_state_repo.clone(),
            get_repo: arc_device_state_repo.clone(),
            update_repo: arc_device_state_repo.clone(),
            delete_repo: arc_device_state_repo,
        });
        let device_events_service = Arc::new(ManageEventService {
            create_repo: arc_event_repo.clone(),
            get_repo: arc_event_repo,
        });

        Ok(FullPostgresAppOutbound {
            device_service,
            device_state_service,
            device_events_service,
        })
    }
}

impl AppOutbound for FullPostgresAppOutbound {
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
}

