use std::sync::Arc;

use crate::application::{ports::outbound::{device_repository::DeviceRepository, device_state_repository::DeviceStateRepository, event_repository::EventRepository}, usecases::{manage_device::ManageDeviceService, manage_device_state::ManageDeviceStateService, manage_event::ManageEventService}};

pub trait AppState: Send + Sync {
    fn get_device_service(&self) -> &Arc<ManageDeviceService<impl DeviceRepository>>;
    fn get_device_state_service(&self) -> &Arc<ManageDeviceStateService<impl DeviceStateRepository>>;
    fn get_event_service(&self) -> &Arc<ManageEventService<impl EventRepository>>;
}

pub trait App {
    async fn start_with_state<AS: AppState+'static>(&self, state: AS) -> Result<(), String>;
}