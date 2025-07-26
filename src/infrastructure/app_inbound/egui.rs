use crate::{application::ports::app::{AppInbound, AppOutbound}, infrastructure::ui::inbound::egui_app::EguiApp};

pub struct EguiAppInbound;

impl EguiAppInbound {
    pub fn new() -> Self {
        EguiAppInbound {}
    }
}

impl AppInbound for EguiAppInbound {
    async fn start_with_outbound<AO: AppOutbound + 'static>(&self, outbound: AO) -> Result<(), String> {
        let native_options = eframe::NativeOptions::default();
        eframe::run_native(
            "Device Platform",
            native_options,
            Box::new(|_| {
                Ok(Box::new(EguiApp::new(outbound)))
            }),
        ).expect(&format!("Failed to start Egui app"));
        Ok(())
    }
}

