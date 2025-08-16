use crate::{
    application::ports::app::AppOutbound,
    domain::device::Device,
    infrastructure::ui::inbound::{
        LoadingStatus, egui_app::try_lock_until_success, managers::device_manager::DeviceManager,
    },
};

pub fn display_device_panel<AO: AppOutbound + 'static>(
    ui: &mut egui::Ui,
    device_manager: &mut DeviceManager,
    outbound: AO,
    must_refresh: bool,
) {
    ui.label("List of devices :");
    let device_list = device_manager.get_device_list();
    let mut should_load = false;
    let device_list_lock = try_lock_until_success(&device_list);
    match device_list_lock.clone() {
        LoadingStatus::Success(devices) => {
            display_cards(ui, devices);
        }
        LoadingStatus::InProgress(Some(devices)) => {
            display_cards(ui, devices);
        }
        LoadingStatus::InProgress(None) => {
            ui.label("Loading devices...");
        }
        LoadingStatus::NotStarted => {
            should_load = true;
        }
        LoadingStatus::Failed(msg) => {
            ui.label(format!("Failed to load devices: {}", msg));
        }
    };
    drop(device_list_lock);
    if must_refresh || should_load {
        device_manager.load_devices(outbound);
    }
}

fn display_cards(ui: &mut egui::Ui, devices: Vec<Device>) {
    egui::ScrollArea::horizontal().show(ui, |ui| {
        ui.horizontal_top(|ui| {
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
                                    ui.heading(device.name());
                                    ui.label(format!("🔢 ID : {}", device.id()));
                                    ui.separator();
                                    for (key, value) in device.events() {
                                        ui.label(format!("Event: {key}"));
                                        ui.label(format!("- 📦 Données : {:?}", value.payload()));
                                        ui.label(format!("- 🧾 Format : {:?}", value.format()));
                                    }
                                });
                            });
                    },
                );

                ui.add_space(12.0); // Espace entre les cartes
            }
        });
    });
}
