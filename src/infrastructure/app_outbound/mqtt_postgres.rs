use std::{env::VarError, sync::Arc};

use crate::{
    application::{
        ports::{
            app::AppOutbound, outbound::{
                device_repository::{
                    CreateDeviceRepository, DeleteDeviceRepository,
                    GetDeviceRepository, UpdateDeviceRepository,
                }, device_state_repository::{CreateDeviceStateRepository, DeleteDeviceStateRepository, GetDeviceStateRepository, UpdateDeviceStateRepository}, event_repository::{CreateEventRepository, GetEventRepository}
            }
        },
        usecases::{
            manage_device::ManageDeviceService, manage_device_state::ManageDeviceStateService,
            manage_event::ManageEventService,
        },
    },
    infrastructure::{
        db::postgres::{
            device_repository::PostgresDeviceRepository, device_state_repository::PostgresDeviceStateRepository, event_repository::PostgresEventRepository
        },
        mqtt::outbound::{
            device_repository::MqttDeviceRepository,
            device_state_repository::MqttDeviceStateRepository,
            event_repository::MqttEventRepository,
        },
    },
};

pub struct MqttAppOutbound {
    device_service: Arc<
        ManageDeviceService<
            MqttDeviceRepository,
            PostgresDeviceRepository,
            MqttDeviceRepository,
            MqttDeviceRepository,
        >,
    >,
    device_state_service: Arc<ManageDeviceStateService<MqttDeviceStateRepository, PostgresDeviceStateRepository, MqttDeviceStateRepository, MqttDeviceStateRepository>>,
    device_events_service: Arc<ManageEventService<MqttEventRepository, PostgresEventRepository>>,
}

impl MqttAppOutbound {
    pub async fn new() -> Result<Self, VarError> {
        todo!()
    }
}

impl AppOutbound for MqttAppOutbound {
    fn get_device_state_service(
        &self,
    ) -> &Arc<ManageDeviceStateService<impl CreateDeviceStateRepository, impl GetDeviceStateRepository, impl UpdateDeviceStateRepository, impl DeleteDeviceStateRepository>> {
        &self.device_state_service
    }

    fn get_event_service(
        &self,
    ) -> &Arc<ManageEventService<impl CreateEventRepository, impl GetEventRepository>> {
        &self.device_events_service
    }

    fn get_device_service(
        &self,
    ) -> &Arc<
        ManageDeviceService<
            impl CreateDeviceRepository,
            impl GetDeviceRepository,
            impl UpdateDeviceRepository,
            impl DeleteDeviceRepository,
        >,
    > {
        &self.device_service
    }
}
