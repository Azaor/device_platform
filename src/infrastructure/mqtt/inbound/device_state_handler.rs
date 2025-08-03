use std::{collections::HashMap, str::FromStr};

use rumqttc::Publish;
use serde_json::Value;
use uuid::Uuid;

use crate::{
    application::ports::{app::AppOutbound, inbound::device_state_service::DeviceStateService}, domain::event::EventDataValue, infrastructure::mqtt::{
        inbound::error::HandlerError,
        mqtt_messages::{
            CreateDeviceStatePayload, DeleteDeviceStatePayload, MqttActionType, MqttMessage,
        },
    }
};

pub async fn handle_device_state<AO: AppOutbound + 'static>(
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
            handle_create_device_state(payload, state).await
        }
        MqttActionType::Delete => {
            let payload = serde_json::from_value(data.payload).map_err(|e| {
                HandlerError::ParsingError(format!("Invalid payload: {}", e.to_string()))
            })?;
            handle_delete_device_state(payload, state).await
        }
        MqttActionType::Update => {
            let payload = serde_json::from_value(data.payload).map_err(|e| {
                HandlerError::ParsingError(format!("Invalid payload: {}", e.to_string()))
            })?;
            handle_update_device_state(payload, state).await
        }
    }
}

async fn handle_create_device_state<AO: AppOutbound + 'static>(
    device_state: CreateDeviceStatePayload,
    state: &AO,
) -> Result<(), HandlerError> {
    let device_state_service = state.get_device_state_service();
    let device_id = Uuid::from_str(&device_state.device_id)
        .map_err(|_| HandlerError::ParsingError("invalid Uuid format".to_string()))?;
    let mut values = HashMap::new();
    for (k, v) in device_state.values{
        let val = EventDataValue::try_from(v).map_err(|_| HandlerError::ParsingError(format!("Invalid data received for key {}", k)))?;
        values.insert(k, val);
    }
    device_state_service
        .create_device_state(device_id, values)
        .await?;
    Ok(())
}

async fn handle_delete_device_state<AO: AppOutbound + 'static>(
    device_state: DeleteDeviceStatePayload,
    state: &AO,
) -> Result<(), HandlerError> {
    let device_state_service = state.get_device_state_service();
    let device_id = Uuid::from_str(&device_state.device_id)
        .map_err(|_| HandlerError::ParsingError("invalid Uuid format".to_string()))?;

    device_state_service.delete_device_state(device_id).await?;
    Ok(())
}

async fn handle_update_device_state<AO: AppOutbound + 'static>(
    device_state: CreateDeviceStatePayload,
    state: &AO,
) -> Result<(), HandlerError> {
    let device_state_service = state.get_device_state_service();
    let device_id = Uuid::from_str(&device_state.device_id)
        .map_err(|_| HandlerError::ParsingError("invalid Uuid format".to_string()))?;
    let mut values = HashMap::new();
    for (k, v) in device_state.values{
        let val = EventDataValue::try_from(v).map_err(|_| HandlerError::ParsingError(format!("Invalid data received for key {}", k)))?;
        values.insert(k, val);
    }
    device_state_service
        .update_device_state(device_id, values)
        .await?;
    Ok(())
}
