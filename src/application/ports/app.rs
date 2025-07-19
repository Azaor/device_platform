use std::sync::Arc;

use crate::application::{
    ports::outbound::{
        device_repository::{
            CreateDeviceRepository, DeleteDeviceRepository, GetDeviceRepository,
            UpdateDeviceRepository,
        },
        device_state_repository::{CreateDeviceStateRepository, DeleteDeviceStateRepository, GetDeviceStateRepository, UpdateDeviceStateRepository}, event_repository::{CreateEventRepository, GetEventRepository},
    },
    usecases::{
        manage_device::ManageDeviceService, manage_device_state::ManageDeviceStateService,
        manage_event::ManageEventService,
    },
};

pub trait AppOutbound: Send + Sync {
    fn get_device_service(
        &self,
    ) -> &Arc<
        ManageDeviceService<
            impl CreateDeviceRepository,
            impl GetDeviceRepository,
            impl UpdateDeviceRepository,
            impl DeleteDeviceRepository,
        >,
    >;
    fn get_device_state_service(
        &self,
    ) -> &Arc<ManageDeviceStateService<impl CreateDeviceStateRepository, impl GetDeviceStateRepository, impl UpdateDeviceStateRepository, impl DeleteDeviceStateRepository>>;
    fn get_event_service(&self) -> &Arc<ManageEventService<impl CreateEventRepository, impl GetEventRepository>>;
}

pub trait AppInbound {
    async fn start_with_outbound<AO: AppOutbound + 'static>(&self, outbound: AO) -> Result<(), String>;
}
