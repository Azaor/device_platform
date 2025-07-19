use crate::application::ports::app::AppOutbound;
#[cfg(feature = "postgres_outbound")]
use crate::infrastructure::app_outbound::full_postgres::FullPostgresAppOutbound;
#[cfg(feature = "in_memory")]
use crate::infrastructure::app_outbound::in_memory::InMemoryAppOutbound;
#[cfg(feature = "mqtt_outbound")]
use crate::infrastructure::app_outbound::mqtt_postgres::MqttAppOutbound;

#[cfg(feature = "postgres_outbound")]
pub mod full_postgres;
#[cfg(feature = "mqtt_outbound")]
pub mod mqtt_postgres;
#[cfg(feature = "in_memory_outbound")]
pub mod in_memory;

pub async fn get_app_outbound() -> impl AppOutbound {
    #[cfg(feature = "in_memory")]
    let app_outbound = InMemoryAppOutbound::new();
    #[cfg(feature = "mqtt_outbound")]
    let app_outbound = MqttAppOutbound::new().await.expect("Failed to create MqttAppOutbound");
    #[cfg(feature = "postgres_outbound")]
    let app_outbound = FullPostgresAppOutbound::new().await
        .expect("Failed to create PostgresAppState");

    return app_outbound
}