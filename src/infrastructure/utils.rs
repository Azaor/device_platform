use std::env::VarError;

#[cfg(feature = "mqtt_outbound")]
use rumqttc::{AsyncClient, EventLoop, MqttOptions};

#[cfg(feature = "postgres")]
pub struct PostgresConfig {
    pub database_url: String,
    pub database_name: String,
    pub username: String,
    pub password: String,
}

#[cfg(feature = "postgres")]
pub fn load_postgres_config_from_env() -> Result<PostgresConfig, VarError> {
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

#[cfg(feature = "postgres")]
pub async fn create_pool(config: PostgresConfig) -> sqlx::Pool<sqlx::Postgres> {
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

#[cfg(feature = "mqtt")]
pub struct MQTTConfig {
    pub mqtt_url: String,
    pub mqtt_port: u16,
    pub device_topic: String,
    pub device_state_topic: String,
    pub event_topic: String,
}

#[cfg(feature = "mqtt")]
pub fn load_mqtt_config_from_env() -> Result<MQTTConfig, VarError> {
    // Load database connection options from environment variables or configuration files
    let mqtt_url = std::env::var(format!("MQTT_URL"))?;
    let mqtt_port = std::env::var(format!("MQTT_PORT"))?;
    let device_topic = std::env::var(format!("MQTT_DEVICE_TOPIC"))?;
    let device_state_topic = std::env::var(format!("MQTT_DEVICE_STATE_TOPIC"))?;
    let event_topic = std::env::var(format!("MQTT_EVENT_TOPIC"))?;
    Ok(MQTTConfig {
        mqtt_url,
        mqtt_port: mqtt_port.parse().map_err(|_| VarError::NotPresent)?,
        device_topic,
        device_state_topic,
        event_topic,
    })
}

#[cfg(feature = "mqtt_outbound")]
pub async fn create_mqtt_client(config: &MQTTConfig) -> (AsyncClient, EventLoop) {
    // Create a connection pool for the Postgres database
    let mqttoptions = MqttOptions::new("rumqtt-async", config.mqtt_url.clone(), config.mqtt_port);
    return AsyncClient::new(mqttoptions, 10)
}

#[cfg(feature = "reqwest")]
pub struct HttpConfig {
    pub base_url: String,
    pub device_create_path: Option<String>,
    pub device_update_path: Option<String>,
    pub device_get_path: Option<String>,
    pub device_delete_path: Option<String>,
    pub device_state_create_path: Option<String>,
    pub device_state_update_path: Option<String>,
    pub device_state_get_path: Option<String>,
    pub device_state_delete_path: Option<String>,
    pub event_create_path: Option<String>,
    pub event_get_path: Option<String>,
}

#[cfg(feature = "reqwest")]
pub fn load_http_config_from_env() -> Result<HttpConfig, VarError> {
    // Load database connection options from environment variables or configuration files
    let base_url = std::env::var("HTTP_BASE_URL")?;
    let device_create_path = std::env::var("HTTP_DEVICE_CREATE_PATH").ok();
    let device_update_path = std::env::var("HTTP_DEVICE_UPDATE_PATH").ok();
    let device_get_path = std::env::var("HTTP_DEVICE_GET_PATH").ok();
    let device_delete_path = std::env::var("HTTP_DEVICE_DELETE_PATH").ok();
    let device_state_create_path = std::env::var("HTTP_DEVICE_STATE_CREATE_PATH").ok();
    let device_state_update_path = std::env::var("HTTP_DEVICE_STATE_UPDATE_PATH").ok();
    let device_state_get_path = std::env::var("HTTP_DEVICE_STATE_GET_PATH").ok();
    let device_state_delete_path = std::env::var("HTTP_DEVICE_STATE_DELETE_PATH").ok();
    let event_create_path = std::env::var("HTTP_EVENT_CREATE_PATH").ok();
    let event_get_path = std::env::var("HTTP_EVENT_GET_PATH").ok();

    Ok(HttpConfig {
        base_url,
        device_create_path,
        device_update_path,
        device_get_path,
        device_delete_path,
        device_state_create_path,
        device_state_update_path,
        device_state_get_path,
        device_state_delete_path,
        event_create_path,
        event_get_path,
    })
}