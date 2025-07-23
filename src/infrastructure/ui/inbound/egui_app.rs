use crate::infrastructure::ui::inbound::LoadingStatus;
use crate::{
    application::ports::app::AppOutbound,
    infrastructure::ui::inbound::device_manager::DeviceManager,
};

#[derive(PartialEq, Eq)]
enum Tab {
    Device,
    DeviceState,
    Event,
}

pub struct EguiApp<AO: AppOutbound> {
    device_manager: DeviceManager, // Example field to hold device names
    outbound: AO,
    selected_tab: Tab,
}

impl<AO: AppOutbound> EguiApp<AO> {
    pub fn new(outbound: AO) -> Self {
        EguiApp {
            device_manager: DeviceManager::new(),
            outbound,
            selected_tab: Tab::Device,
        }
    }
}

impl<AO: AppOutbound + 'static> eframe::App for EguiApp<AO> {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Device Platform");
            ui.horizontal(|ui| {
                if ui
                    .selectable_label(self.selected_tab == Tab::Device, "Device")
                    .clicked()
                {
                    self.selected_tab = Tab::Device;
                }
                if ui
                    .selectable_label(self.selected_tab == Tab::DeviceState, "Device State")
                    .clicked()
                {
                    self.selected_tab = Tab::DeviceState;
                }
                if ui
                    .selectable_label(self.selected_tab == Tab::Event, "Events")
                    .clicked()
                {
                    self.selected_tab = Tab::Event;
                }
            });
            ui.separator();
            ui.label("List of devices :");
            let device_list = self.device_manager.get_device_list();
            let mut should_repaint = false;
            match device_list.lock().unwrap().as_ref() {
                Ok(devices) => {
                    egui::ScrollArea::horizontal()
                        .auto_shrink([false; 2])
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                for device in devices {
                                    // Limiter la largeur de chaque carte
                                    let card_size = egui::vec2(200.0, 150.0); // largeur x hauteur approx

                                    ui.allocate_ui_with_layout(
                                        card_size,
                                        egui::Layout::top_down(egui::Align::Min),
                                        |ui| {
                                            egui::Frame::group(ui.style())
                                                .fill(ui.visuals().panel_fill)
                                                .stroke(egui::Stroke::new(
                                                    1.0,
                                                    ui.visuals().widgets.inactive.bg_fill,
                                                ))
                                                .corner_radius(egui::CornerRadius::same(8))
                                                .inner_margin(egui::Margin::symmetric(10, 8))
                                                .show(ui, |ui| {
                                                    ui.vertical(|ui| {
                                                        ui.heading(&device.name);
                                                        ui.separator();
                                                        ui.label(format!("🔢 ID : {}", device.id));
                                                        ui.label(format!(
                                                            "📦 Données : {:?}",
                                                            device.event_data
                                                        ));
                                                        ui.label(format!(
                                                            "🧾 Format : {:?}",
                                                            device.event_format
                                                        ));
                                                    });
                                                });
                                        },
                                    );

                                    ui.add_space(12.0); // Espace entre les cartes
                                }
                            });
                        });
                }
                Err(status) => {
                    match status {
                        LoadingStatus::NotStarted => {
                            ui.label("Loading devices...");
                            should_repaint = true;
                        }
                        LoadingStatus::InProgress => {
                            ui.label("Loading devices...");
                            should_repaint = true;
                        }
                        LoadingStatus::Failed(msg) => {
                            ui.label(format!("Failed to load devices: {}", msg));
                        }
                    };
                }
            };
            if should_repaint {
                ctx.request_repaint();
            }
            if ui.button("Refresh Devices").clicked() {
                self.device_manager.load_devices(self.outbound.clone());
            }
        });
    }
}
