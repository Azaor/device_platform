use chrono::Utc;
use rumqttc::Publish;
use uuid::Uuid;

use crate::{
    application::ports::{
        app::AppOutbound,
        inbound::{
            device_service::{DeviceService, DeviceServiceError},
            device_state_service::{DeviceStateService, DeviceStateServiceError},
            event_service::{EventService, EventServiceError},
        },
    }, domain::event::Event, infrastructure::mqtt::inbound::error::HandlerError
};

pub async fn handle_event<AO: AppOutbound + 'static>(
    received: &Publish,
    state: &AO,
) -> Result<(), HandlerError> {
    // Handle incoming messages
    let device_id = received
        .topic
        .split('/')
        .nth(1)
        .unwrap_or("unknown")
        .to_string();
    // device Id to UUID conversion
    let device_id = match Uuid::parse_str(&device_id) {
        Ok(id) => id,
        Err(_) => return Err(HandlerError::ParsingError("Invalid UUID".to_string())), // Skip if device_id is not valid UUID
    };
    let device_service = state.get_device_service();
    let event_service = state.get_event_service();
    let device_state_service = state.get_device_state_service();
    let device = match device_service.get_device(device_id).await {
        Ok(Some(device)) => device,
        Ok(None) => return Err(HandlerError::ParsingError("Device not found".to_string())), // Skip if device not found
        Err(DeviceServiceError::NotFound) => {
            return Err(HandlerError::ParsingError("Device not found".to_string()));
        } // Skip if device not found
        Err(_) => {
            return Err(HandlerError::InternalError(
                "Internal Error occured while fetching device".to_string(),
            ));
        }
    };

    let event = Event::new(&device, &Utc::now(), &received.payload)?;
    match event_service.handle_event(event.clone()).await {
        Ok(_) => (),
        Err(EventServiceError::InternalError(err)) => {
            return Err(HandlerError::InternalError(format!(
                "An internal error occured while inserting event: {}",
                err
            )));
        }
        Err(EventServiceError::InvalidInput(err)) => {
            return Err(HandlerError::ParsingError(format!(
                "Invalid event format: {}",
                err
            )));
        }
    }
    match device_state_service
        .create_device_state(device_id, event.payload)
        .await
    {
        Ok(_) => (),
        Err(err) => match err {
            DeviceStateServiceError::InternalError => {
                return Err(HandlerError::InternalError(format!(
                    "An internal error occured while inserting device state."
                )));
            }
            DeviceStateServiceError::DeviceNotFound => {
                return Err(HandlerError::ParsingError(
                    "Device state not found".to_string(),
                ));
            }
            DeviceStateServiceError::AlreadyExists => {
                return Err(HandlerError::ParsingError(
                    "Device state already exists".to_string(),
                ));
            }
            _ => {
                return Err(HandlerError::InternalError(
                    "An unknown error occured while inserting device state.".to_string(),
                ));
            }
        },
    }
    Ok(())
}
