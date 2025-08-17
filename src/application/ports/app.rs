use std::{fmt::Debug, sync::Arc};

use crate::application::{
    ports::outbound::{
        action_repository::{CreateActionRepository, HandleActionRepository},
        device_repository::{
            CreateDeviceRepository, DeleteDeviceRepository, GetDeviceRepository,
            UpdateDeviceRepository,
        },
        device_state_repository::{
            CreateDeviceStateRepository, DeleteDeviceStateRepository, GetDeviceStateRepository,
            UpdateDeviceStateRepository,
        },
        event_repository::{CreateEventRepository, GetEventRepository},
    },
    usecases::{
        manage_action::ManageActionService, manage_device::ManageDeviceService,
        manage_device_state::ManageDeviceStateService, manage_event::ManageEventService,
    },
};

pub trait AppOutbound: Send + Sync + Clone + Debug {
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
    ) -> &Arc<
        ManageDeviceStateService<
            impl CreateDeviceStateRepository,
            impl GetDeviceStateRepository,
            impl UpdateDeviceStateRepository,
            impl DeleteDeviceStateRepository,
        >,
    >;
    fn get_event_service(
        &self,
    ) -> &Arc<ManageEventService<impl CreateEventRepository, impl GetEventRepository>>;
    fn get_action_service(
        &self,
    ) -> &Arc<ManageActionService<impl CreateActionRepository, impl HandleActionRepository>>;
}

pub trait AppInbound {
    async fn start_with_outbound<AO: AppOutbound + 'static>(
        &self,
        outbound: AO,
    ) -> Result<(), String>;
}
