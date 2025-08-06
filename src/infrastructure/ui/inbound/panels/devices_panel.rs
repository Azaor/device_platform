use crate::{application::ports::app::AppOutbound, infrastructure::ui::inbound::{managers::device_manager::DeviceManager, LoadingStatus}};

pub fn display_device_panel<AO: AppOutbound+'static>(ui: &mut egui::Ui, device_manager: &mut DeviceManager, outbound: AO) -> bool{
    ui.label("List of devices :");
    let device_list = device_manager.get_device_list();
    let mut should_repaint = false;
    let mut should_load = false;
    match device_list.lock().unwrap().as_ref() {
        Ok(devices) => {
            egui::ScrollArea::horizontal()
                .show(ui, |ui| {
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
                    should_load = true;
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
    if should_load {
        device_manager.load_devices(outbound);
        should_repaint = true;
    }
    return should_repaint;
}