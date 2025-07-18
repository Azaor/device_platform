use std::{env::VarError, sync::Arc};

use crate::{
    application::{
        ports::{
            app::AppState,
            outbound::{
                device_repository::DeviceRepository, device_state_repository::DeviceStateRepository, event_repository::EventRepository,
            },
        },
        usecases::{
            manage_device::ManageDeviceService, manage_device_state::ManageDeviceStateService,
            manage_event::ManageEventService,
        },
    },
    infrastructure::db::postgres::{
            device_repository::PostgresDeviceRepository,
            device_state_repository::PostgresDeviceStateRepository, event_repository::PostgresEventRepository,
        },
};

pub struct PostgresAppState {
    device_service: Arc<ManageDeviceService<PostgresDeviceRepository>>,
    device_state_service: Arc<ManageDeviceStateService<PostgresDeviceStateRepository>>,
    device_events_service: Arc<ManageEventService<PostgresEventRepository>>,
}

impl PostgresAppState {
    pub async fn new() -> Result<Self, VarError> {
        // Load env files or environment variables
        dotenv::dotenv().ok();
        // Load configuration from environment variables or configuration files
        let postgres_config = load_config_from_env()?;
        let pool = create_pool(postgres_config).await;
        let device_repo = PostgresDeviceRepository::new(pool.clone()).await;
        let device_state_repo = PostgresDeviceStateRepository::new(pool.clone()).await;
        let event_repo = PostgresEventRepository::new(pool.clone()).await;

        // Initialize the repositories
        device_repo.init().await;
        device_state_repo.init().await;
        event_repo.init().await;

        let device_service = Arc::new(ManageDeviceService { repo: device_repo });
        let device_state_service = Arc::new(ManageDeviceStateService {
            repo: device_state_repo,
        });
        let device_events_service = Arc::new(ManageEventService {
            event_repository: event_repo,
        });

        Ok(PostgresAppState {
            device_service,
            device_state_service,
            device_events_service,
        })
    }
}

impl AppState for PostgresAppState {
    fn get_device_service(&self) -> &Arc<ManageDeviceService<impl DeviceRepository>> {
        &self.device_service
    }

    fn get_device_state_service(
        &self,
    ) -> &Arc<ManageDeviceStateService<impl DeviceStateRepository>> {
        &self.device_state_service
    }

    fn get_event_service(&self) -> &Arc<ManageEventService<impl EventRepository>> {
        &self.device_events_service
    }
}
pub struct PostgresConfig {
    pub database_url: String,
    pub database_name: String,
    pub username: String,
    pub password: String,
}

fn load_config_from_env() -> Result<PostgresConfig, VarError> {
    // Load database connection options from environment variables or configuration files
    let database_url = std::env::var(format!("DATABASE_URL"))?;
    let database_name = std::env::var(format!("DATABASE_NAME"))?;
    let username = std::env::var(format!("DATABASE_USERNAME"))?;
    let password = std::env::var(format!("DATABASE_PASSWORD"))?;
    Ok(PostgresConfig {
        database_url,
        database_name,
        username,
        password,
    })
}

async fn create_pool(config: PostgresConfig) -> sqlx::Pool<sqlx::Postgres> {
    // Create a connection pool for the Postgres database
    sqlx::PgPool::connect_with(
        sqlx::postgres::PgConnectOptions::new()
            .host(&config.database_url)
            .port(5432)
            .database(&config.database_name)
            .username(&config.username)
            .password(&config.password),
    )
    .await
    .expect("Failed to connect to Postgres database")
}