
use std::{sync::{Arc, Mutex, MutexGuard}, time::Duration};

use crate::{
    application::ports::app::AppOutbound, infrastructure::ui::inbound::{managers::{device_manager::DeviceManager, device_state_manager::DeviceStateManager}, panels::{device_state_panel::display_device_state_panel, devices_panel::display_device_panel}},
};

#[derive(PartialEq, Eq)]
enum Tab {
    Device,
    DeviceState,
    Event,
}

pub struct EguiApp<AO: AppOutbound> {
    device_manager: DeviceManager,
    device_state_manager: DeviceStateManager,
    outbound: AO,
    selected_tab: Tab,
    last_repaint: std::time::Instant,
}

impl<AO: AppOutbound> EguiApp<AO> {
    pub fn new(outbound: AO) -> Self {
        EguiApp {
            device_manager: DeviceManager::new(),
            device_state_manager: DeviceStateManager::new(),
            outbound,
            selected_tab: Tab::Device,
            last_repaint: std::time::Instant::now()
        }
    }
}

impl<AO: AppOutbound + 'static> eframe::App for EguiApp<AO> {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        println!("repaint: {:?}", self.last_repaint.elapsed());
        ctx.request_repaint_after(Duration::from_secs(1));
        let mut must_refresh = false;
        if self.last_repaint.elapsed().as_secs() > 1 {
            self.last_repaint = std::time::Instant::now();
            must_refresh = true;
        }
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
            if self.selected_tab == Tab::Device {
                display_device_panel(ui, &mut self.device_manager, self.outbound.clone(), must_refresh);
            } else if self.selected_tab == Tab::DeviceState {
                display_device_state_panel(ui, &mut self.device_state_manager, self.outbound.clone(), must_refresh);
            } else if self.selected_tab == Tab::Event {
                ui.label("Events Panel (not implemented yet)");
            }
        });
    }
}

pub fn try_lock_until_success<T: Clone>(mutex: &Arc<Mutex<T>>) -> MutexGuard<'_, T> {
    loop {
        match mutex.lock() {
            Ok(guard) => return guard,
            Err(_) => {
                mutex.clear_poison();
            }
        }
    }
}