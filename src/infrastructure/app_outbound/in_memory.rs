use std::sync::Arc;

use tokio::sync::Mutex;

use crate::{
    application::{
        ports::{
            app::AppOutbound,
            outbound::{
                action_repository::{CreateActionRepository, HandleActionRepository},
                device_repository::{
                    CreateDeviceRepository, DeleteDeviceRepository, GetDeviceRepository,
                    UpdateDeviceRepository,
                },
                device_state_repository::{
                    CreateDeviceStateRepository, DeleteDeviceStateRepository,
                    GetDeviceStateRepository, UpdateDeviceStateRepository,
                },
                event_repository::{CreateEventRepository, GetEventRepository},
            },
        },
        usecases::{
            manage_action::ManageActionService, manage_device::ManageDeviceService,
            manage_device_state::ManageDeviceStateService, manage_event::ManageEventService,
        },
    },
    infrastructure::db::memory::{
        action_repository::InMemoryActionRepository, device_repository::InMemoryDeviceRepository,
        device_state_repository::InMemoryDeviceStateRepository,
        event_repository::InMemoryEventRepository,
    },
};

#[derive(Debug, Clone)]
pub struct InMemoryAppOutbound {
    device_service: Arc<
        ManageDeviceService<
            InMemoryDeviceRepository,
            InMemoryDeviceRepository,
            InMemoryDeviceRepository,
            InMemoryDeviceRepository,
        >,
    >,
    device_state_service: Arc<
        ManageDeviceStateService<
            InMemoryDeviceStateRepository,
            InMemoryDeviceStateRepository,
            InMemoryDeviceStateRepository,
            InMemoryDeviceStateRepository,
        >,
    >,
    device_events_service:
        Arc<ManageEventService<InMemoryEventRepository, InMemoryEventRepository>>,
    device_actions_service:
        Arc<ManageActionService<InMemoryActionRepository, InMemoryActionRepository>>,
}

impl InMemoryAppOutbound {
    pub fn new() -> Self {
        let device_repo = InMemoryDeviceRepository::new();
        let device_state_repo = InMemoryDeviceStateRepository::new();
        let event_repo = InMemoryEventRepository::new();
        let action_repo = InMemoryActionRepository::new();

        let arc_event_repo = Arc::new(event_repo);
        let arc_device_repo = Arc::new(device_repo);
        let arc_device_state_repo = Arc::new(device_state_repo);
        let arc_action_repo = Arc::new(Mutex::new(action_repo));

        let device_service = Arc::new(ManageDeviceService {
            create_repo: arc_device_repo.clone(),
            get_repo: arc_device_repo.clone(),
            update_repo: arc_device_repo.clone(),
            delete_repo: arc_device_repo,
        });
        let device_state_service = Arc::new(ManageDeviceStateService {
            create_repo: arc_device_state_repo.clone(),
            get_repo: arc_device_state_repo.clone(),
            update_repo: arc_device_state_repo.clone(),
            delete_repo: arc_device_state_repo,
        });
        let device_events_service = Arc::new(ManageEventService {
            create_repo: arc_event_repo.clone(),
            get_repo: arc_event_repo,
        });
        let device_actions_service = Arc::new(ManageActionService {
            create_repo: arc_action_repo.clone(),
            get_repo: arc_action_repo,
        });
        InMemoryAppOutbound {
            device_service,
            device_state_service,
            device_events_service,
            device_actions_service,
        }
    }
}

impl AppOutbound for InMemoryAppOutbound {
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
        return &self.device_service;
    }

    fn get_device_state_service(
        &self,
    ) -> &Arc<
        ManageDeviceStateService<
            impl CreateDeviceStateRepository,
            impl GetDeviceStateRepository,
            impl UpdateDeviceStateRepository,
            impl DeleteDeviceStateRepository,
        >,
    > {
        return &self.device_state_service;
    }

    fn get_event_service(
        &self,
    ) -> &Arc<ManageEventService<impl CreateEventRepository, impl GetEventRepository>> {
        return &self.device_events_service;
    }

    fn get_action_service(
        &self,
    ) -> &Arc<ManageActionService<impl CreateActionRepository, impl HandleActionRepository>> {
        return &self.device_actions_service;
    }
}
