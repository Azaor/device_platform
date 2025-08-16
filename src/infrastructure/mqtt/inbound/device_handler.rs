use std::str::FromStr;

use rumqttc::Publish;
use serde_json::Value;
use uuid::Uuid;

use crate::{
    application::ports::{app::AppOutbound, inbound::device_service::DeviceService},
    domain::device::Device,
    infrastructure::mqtt::{
        inbound::error::HandlerError,
        mqtt_messages::{
            CreateDevicePayload, DeleteDevicePayload, MqttActionType, MqttMessage,
            UpdateDevicePayload, deserialize_event,
        },
    },
};

#[tracing::instrument]
pub async fn handle_device<AO: AppOutbound + 'static>(
    received: &Publish,
    state: &AO,
) -> Result<(), HandlerError> {
    let data: MqttMessage<Value> = serde_json::from_slice(&received.payload)
        .map_err(|e| HandlerError::ParsingError(format!("Invalid payload: {}", e.to_string())))?;
    match data.action_type {
        MqttActionType::Create => {
            let payload = serde_json::from_value(data.payload).map_err(|e| {
                HandlerError::ParsingError(format!("Invalid payload: {}", e.to_string()))
            })?;
            handle_create_device(payload, state).await
        }
        MqttActionType::Delete => {
            let payload = serde_json::from_value(data.payload).map_err(|e| {
                HandlerError::ParsingError(format!("Invalid payload: {}", e.to_string()))
            })?;
            handle_delete_device(payload, state).await
        }
        MqttActionType::Update => {
            let payload = serde_json::from_value(data.payload).map_err(|e| {
                HandlerError::ParsingError(format!("Invalid payload: {}", e.to_string()))
            })?;
            handle_update_device(payload, state).await
        }
    }
}

pub async fn handle_create_device<AO: AppOutbound + 'static>(
    device: CreateDevicePayload,
    state: &AO,
) -> Result<(), HandlerError> {
    let device_service = state.get_device_service();
    let events = deserialize_event(&device.events)?;
    let device = Device::new(
        &Uuid::from_str(&device.id)
            .map_err(|_| HandlerError::ParsingError("invalid Uuid format".to_string()))?,
        &device.physical_id,
        &Uuid::from_str(&device.user_id)
            .map_err(|_| HandlerError::ParsingError("invalid Uuid format".to_string()))?,
        &device.name,
        events,
    );
    device_service.create_device(&device).await?;
    Ok(())
}

pub async fn handle_delete_device<AO: AppOutbound + 'static>(
    device: DeleteDevicePayload,
    state: &AO,
) -> Result<(), HandlerError> {
    let device_service = state.get_device_service();
    let device_id = Uuid::from_str(&device.id)
        .map_err(|_| HandlerError::ParsingError("invalid Uuid format".to_string()))?;
    device_service.delete_device(device_id).await?;
    Ok(())
}

pub async fn handle_update_device<AO: AppOutbound + 'static>(
    device: UpdateDevicePayload,
    state: &AO,
) -> Result<(), HandlerError> {
    let device_service = state.get_device_service();
    let device_id = Uuid::from_str(&device.id)
        .map_err(|_| HandlerError::ParsingError("invalid Uuid format".to_string()))?;
    let events = deserialize_event(&device.events)?;
    device_service
        .update_device(
            device_id,
            Some(device.physical_id),
            Some(device.name),
            Some(events),
        )
        .await?;
    Ok(())
}
