use crate::application::ports::app::AppInbound;
#[cfg(feature = "axum")]
use crate::infrastructure::app_inbound::axum::AxumAppInbound;
#[cfg(feature = "mqtt_inbound")]
use crate::infrastructure::app_inbound::mqtt::MQTTAppInbound;

#[cfg(feature = "axum")]
pub mod axum;

#[cfg(feature = "mqtt_inbound")]
pub mod mqtt;

#[cfg(feature = "egui_inbound")]
pub mod egui;

pub fn get_app_inbound() -> impl AppInbound {
    #[cfg(feature = "axum")]
    let app = AxumAppInbound::new();

    #[cfg(feature = "mqtt_inbound")]
    let app = MQTTAppInbound::new();

    #[cfg(feature = "egui_inbound")]
    let app = egui::EguiAppInbound::new();
    
    return app
}