#[allow(dead_code)]
mod application;
mod domain;
mod infrastructure;

use crate::{
    application::ports::app::AppInbound,
    infrastructure::{app_inbound::get_app_inbound, app_outbound::get_app_outbound},
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    
    let app_outbound = get_app_outbound().await;

    let app_inbound = get_app_inbound();

    match app_inbound.start_with_outbound(app_outbound).await {
        Ok(_) => println!("Application stopped successfully"),
        Err(e) => eprintln!("Failed to run application: {}", e),
    }
}
