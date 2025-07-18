use std::sync::Arc;

use crate::{application::{ports::{app::AppState, outbound::{device_repository::DeviceRepository, device_state_repository::DeviceStateRepository, event_repository::EventRepository}}, usecases::{manage_device::ManageDeviceService, manage_device_state::ManageDeviceStateService, manage_event::ManageEventService}}, infrastructure::db::memory::{device_repository::InMemoryDeviceRepository, device_state_repository::InMemoryDeviceStateRepository, event_repository::InMemoryEventRepository}};

#[derive(Clone)]
pub struct InMemoryAppState {
    device_service: Arc<ManageDeviceService<InMemoryDeviceRepository>>,
    device_state_service: Arc<ManageDeviceStateService<InMemoryDeviceStateRepository>>,
    device_events_service: Arc<ManageEventService<InMemoryEventRepository>>,
}

impl InMemoryAppState {
    pub fn new() -> Self {
        let device_repo = InMemoryDeviceRepository::new();
        let device_state_repo = InMemoryDeviceStateRepository::new();
        let event_repo = InMemoryEventRepository::new();

        let device_service = Arc::new(ManageDeviceService { repo: device_repo });
        let device_state_service = Arc::new(ManageDeviceStateService { repo: device_state_repo });
        let device_events_service = Arc::new(ManageEventService { event_repository: event_repo });

        InMemoryAppState {
            device_service,
            device_state_service,
            device_events_service
        }
    }
}

impl AppState for InMemoryAppState {
    fn get_device_service(&self) -> &Arc<ManageDeviceService<impl DeviceRepository>> {
        return &self.device_service;
    }

    fn get_device_state_service(&self) -> &Arc<ManageDeviceStateService<impl DeviceStateRepository>> {
        return &self.device_state_service;
    }

    fn get_event_service(&self) -> &Arc<ManageEventService<impl EventRepository>> {
        return &self.device_events_service
    }
}