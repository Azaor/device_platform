use std::str::FromStr;

use chrono::{DateTime};
use rumqttc::Publish;
use serde_json::Value;
use uuid::Uuid;

use crate::{
    application::ports::{
        app::AppOutbound,
        inbound::{
            device_service::{DeviceService},
            device_state_service::{DeviceStateService},
            event_service::{EventService},
        },
    }, domain::event::Event, infrastructure::mqtt::{inbound::error::HandlerError, mqtt_messages::{CreateEventPayload, MqttActionType, MqttMessage}}
};

pub async fn handle_event<AO: AppOutbound + 'static>(
    received: &Publish,
    state: &AO,
) -> Result<(), HandlerError> {
    let data: MqttMessage<Value> = serde_json::from_slice(&received.payload).map_err(|e| HandlerError::ParsingError(format!("Invalid payload: {}", e.to_string())))?;
    match data.action_type {
        MqttActionType::Create => {
            let payload = serde_json::from_value(data.payload).map_err(|e| HandlerError::ParsingError(format!("Invalid payload: {}", e.to_string())))?;
            handle_create_event(payload, state).await
        },
        MqttActionType::Delete => return Err(HandlerError::ParsingError("Invalid payload, no delete on events".to_string())),
        MqttActionType::Update => return Err(HandlerError::ParsingError("Invalid payload, no update on events".to_string())),
    }
    
}

async fn handle_create_event<AO: AppOutbound + 'static>(
    event: CreateEventPayload,
    state: &AO,
) -> Result<(), HandlerError> {
    let device_id = Uuid::from_str(&event.device_id).map_err(|_| HandlerError::ParsingError("invalid Uuid format".to_string()))?;
    let timestamp = DateTime::from_str(&event.timestamp).map_err(|_| HandlerError::ParsingError("invalid Uuid format".to_string()))?;
    let device_service = state.get_device_service();
    let event_service = state.get_event_service();
    let device_state_service = state.get_device_state_service();
    let device = match device_service.get_device(device_id).await? {
        Some(device) => device,
        None => return Err(HandlerError::ParsingError("Device not found".to_string())), // Skip if device not found
    };

    let event = Event::new_checked(&device, &timestamp, &event.event_data.as_bytes())?;
    event_service.handle_event(event.clone(), &device.event_format).await?;
    device_state_service
        .create_device_state(device_id, event.payload)
        .await?;
    Ok(())
}