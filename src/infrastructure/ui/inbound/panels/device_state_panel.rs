use std::{collections::HashMap, str::FromStr};

use egui::{RichText, Vec2};
use uuid::Uuid;

use crate::{
    application::ports::app::AppOutbound, domain::event::event_data_value::EventDataValue, infrastructure::ui::inbound::{
        egui_app::try_lock_until_success, managers::device_state_manager::{DeviceStateManager, DisplayableDeviceState}, widgets::gauge::gauge, LoadingStatus
    }
};

pub fn display_device_state_panel<AO: AppOutbound + 'static>(
    ui: &mut egui::Ui,
    device_state_manager: &mut DeviceStateManager,
    outbound: AO,
    must_refresh: bool,
) {
    let user_id = Uuid::from_str("4a78a953-99bc-4a08-932e-956ef3f7d8fc").unwrap(); // Example user ID
    ui.label("List of devices states :");
    let device_states_mtx = device_state_manager.get_device_states();
    let mut should_load = false;
    let device_states_lock = try_lock_until_success(&device_states_mtx);
    match device_states_lock.clone() {
        LoadingStatus::Success(device_states) => {
            display_cards(ui, device_states);
        }
        LoadingStatus::InProgress(Some(device_states)) => {
            display_cards(ui, device_states);
        }
        LoadingStatus::InProgress(None) => {
            ui.label("Loading device states...");
        }
        LoadingStatus::NotStarted => {
            should_load = true;
        }
        LoadingStatus::Failed(e) => {
            ui.label(format!("Failed to load device states : {}", e));
        }
    };
    drop(device_states_lock);
    if must_refresh || should_load {
        println!("Device states loaded");
        let _ = device_state_manager.load_device_state(user_id, outbound);
    }
}

fn display_cards(ui: &mut egui::Ui, device_states: HashMap<Uuid, DisplayableDeviceState>) {
    egui::ScrollArea::horizontal().show(ui, |ui| {
        ui.horizontal_top(|ui| {
            for (_, device_state) in device_states {
                // Limiter la largeur de chaque carte
                let card_size = egui::vec2(400.0, 300.0); // largeur x hauteur approx

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
                                    ui.heading(device_state.device_name);
                                    ui.label(
                                        RichText::new(format!("ID : {}", device_state.device_id))
                                            .italics()
                                            .size(9.),
                                    );
                                    ui.separator();

                                    if device_state.last_update.is_some()
                                        && device_state.values.is_some()
                                    {
                                        let device_state_values = device_state.values.unwrap();
                                        for (key, value) in device_state_values {
                                            ui.label(format!("{}:", key.to_uppercase()));
                                            if let EventDataValue::Number(num) = value {
                                                gauge(ui, num, 0, 50, Vec2::new(250.0, 50.0));
                                            }
                                        }
                                        ui.label(format!(
                                            "🕘 Last timestamp:\n{} ",
                                            device_state
                                                .last_update
                                                .unwrap()
                                                .format("%d/%m/%Y %H:%M:%S")
                                        ));
                                    } else {
                                        ui.label("No state available");
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
