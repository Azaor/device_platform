mod domain;
mod application;
mod infrastructure;

use crate::{
    application::ports::app::App,
};
#[cfg(feature = "axum")]
use infrastructure::http::axum::app::AxumApp;
#[cfg(feature = "in_memory")]
use infrastructure::db::memory::app_state::InMemoryAppState;
#[cfg(feature = "postgres")]
use crate::infrastructure::db::postgres::app_state::PostgresAppState;
#[cfg(feature = "mqtt")]
use infrastructure::mqtt::inbound::app::MQTTApp;


#[tokio::main]
async fn main() {
    #[cfg(feature = "in_memory")]
    let app_state = InMemoryAppState::new();

    #[cfg(feature = "postgres")]
    let app_state = PostgresAppState::new().await
        .expect("Failed to create PostgresAppState");

    #[cfg(feature = "axum")]
    let app = AxumApp::new();

    #[cfg(feature = "mqtt")]
    let app = MQTTApp::new("device/+/state");


    match app.start_with_state(app_state).await {
        Ok(_) => println!("Application stopped successfully"),
        Err(e) => eprintln!("Failed to run application: {}", e),
    }
}
