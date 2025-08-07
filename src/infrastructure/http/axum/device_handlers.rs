use std::{collections::HashMap, str::FromStr, sync::Arc};

use axum::{
    Json,
    extract::{Path, State},
    response::{IntoResponse, Response},
};
use serde::Serialize;
use serde_json::{Map, Value};
use tracing::{error, instrument, trace, warn};
use uuid::Uuid;

use crate::{
    application::ports::{app::AppOutbound, inbound::device_service::{DeviceService, DeviceServiceError}},
    domain::device::{Device, EventDataType, EventFormat},
    infrastructure::{http::axum::error::ErrorResponse, utils::log_device_service_error},
};

#[instrument]
pub async fn create_device_handler<AO: AppOutbound>(
    State(services): State<Arc<AO>>,
    Json(payload): Json<Value>,
) -> Result<Json<DeviceResponse>, Response> {
    let service = services.get_device_service();
    let payload: CreateDeviceRequest = match CreateDeviceRequest::try_from(payload) {
        Ok(req) => req,
        Err(err) => {
            error!(result = "error", details = %err);
            return Err(ErrorResponse {
                    status: 400,
                    message: err,
                }.into_response())
        },
    };
    
    let device = Device::new(
        &Uuid::new_v4(),
        &payload.physical_id,
        &payload.user_id,
        &payload.name.clone(),
        payload.event_format,
        payload.event_data,
    );
    match service.create_device(&device).await {
        Ok(device) => {
            let event_data: HashMap<String, String> = device
                .event_data()
                .clone()
                .into_iter()
                .map(|(k, v)| (k, v.to_string()))
                .collect();
            trace!(result = "success");
            Ok(Json(DeviceResponse {
                id: device.id().clone(),
                physical_id: device.physical_id().to_string(),
                user_id: device.user_id().clone(),
                name: device.name().to_string(),
                event_format: device.event_format().to_string(),
                event_data: event_data,
            }))
        }
        Err(err) => {
            Err(log_and_return_response(err))
        },
    }
    
}

#[instrument]
pub async fn get_device_handler<AO: AppOutbound>(
    State(services): State<Arc<AO>>,
    Path(device_id): Path<String>,
) -> Result<Json<DeviceResponse>, Response> {
    let service = services.get_device_service();
    let id = match Uuid::parse_str(&device_id) {
        Ok(id) => id,
        Err(_) => {
            warn!(result = "warn", details = format!("Invalid device id provided : {}", device_id));
            return Err(ErrorResponse {
                status: 400,
                message: "Invalid device ID".to_string(),
            }
            .into_response());
        }
    };

    match service.get_device(id).await {
        Ok(Some(device)) => {
            let event_data: HashMap<String, String> = device
                .event_data()
                .clone()
                .into_iter()
                .map(|(k, v)| (k, v.to_string()))
                .collect();
            trace!(result = "success");
            Ok(Json(DeviceResponse {
                id: device.id().clone(),
                physical_id: device.physical_id().to_string(),
                user_id: device.user_id().clone(),
                name: device.name().to_string(),
                event_format: device.event_format().to_string(),
                event_data: event_data,
            }))
        }
        Ok(None) => {
            warn!(result = "warn", details = format!("Device with ID {} not found in DB", device_id));
            Err(ErrorResponse {
                status: 404,
                message: "Device not found".to_string(),
            }.into_response())
        },
        Err(err) => Err(log_and_return_response(err))
    }
}

#[instrument]
pub async fn get_devices_handler<AO: AppOutbound>(
    State(services): State<Arc<AO>>,
) -> Result<Json<Vec<DeviceResponse>>, Response> {
    let service = services.get_device_service();
    let uuid = Uuid::from_str("4a78a953-99bc-4a08-932e-956ef3f7d8fc").unwrap();
    match service.get_devices_by_user_id(uuid).await {
        Ok(devices) => {
            let mut device_responses = Vec::new();
            for device in devices {
                let event_data: HashMap<String, String> = device
                    .event_data(    )
                    .clone()
                    .into_iter()
                    .map(|(k, v)| (k, v.to_string()))
                    .collect();
                device_responses.push(DeviceResponse {
                    id: device.id().clone(),
                    physical_id: device.physical_id().to_string(),
                    user_id: device.user_id().clone(),
                    name: device.name().to_string(),
                    event_format: device.event_format().to_string(),
                    event_data: event_data,
                })
            }
            trace!(result = "success");
            Ok(Json(device_responses))
        }
        Err(err) => Err(log_and_return_response(err)),
    }
}

#[instrument]
pub async fn delete_device_handler<AO: AppOutbound>(
    State(services): State<Arc<AO>>,
    Path(device_id): Path<String>,
) -> Result<(), Response> {
    let service = services.get_device_service();
    let id = match Uuid::parse_str(&device_id) {
        Ok(id) => id,
        Err(_) => {
            warn!(result = "warn", details = format!("Invalid device id provided : {}", device_id));
            return Err(ErrorResponse {
                status: 400,
                message: "Invalid device ID".to_string(),
            }
            .into_response());
        }
    };

    match service.delete_device(id).await {
        Ok(_) => {
            trace!(result = "success");
            Ok(())
        },
        Err(err) => Err(log_and_return_response(err)),
    }
}

#[instrument]
pub async fn update_device_handler<AO: AppOutbound>(
    State(services): State<Arc<AO>>,
    Path(device_id): Path<String>,
    Json(payload): Json<Value>,
) -> Result<Json<DeviceResponse>, Response> {
    let service = services.get_device_service();
    let id = match Uuid::parse_str(&device_id) {
        Ok(id) => id,
        Err(_) => {
            warn!(result = "warn", details = format!("Invalid device id provided : {}", device_id));
            return Err(ErrorResponse {
                status: 400,
                message: "Invalid device ID".to_string(),
            }
            .into_response());
        }
    };

    let name = payload
        .get("name")
        .and_then(Value::as_str)
        .map(String::from);
    let event_data_raw = payload
        .get("event_data")
        .and_then(Value::as_object);
    let mut event_data = None;
    if let Some(data) = event_data_raw {
        event_data = Some(parse_event_data(data).map_err(|e| {
            warn!(result = "warn", details = %e);
            ErrorResponse {
                status: 400,
                message: e,
            }
        })?);
    }

    match service.update_device(id, name, event_data).await {
        // convert event_data to HashMap<String, String>
        Ok(device) => {
            let event_data: HashMap<String, String> = device
                .event_data()
                .clone()
                .into_iter()
                .map(|(k, v)| (k, v.to_string()))
                .collect();
            trace!(result = "success");
            Ok(Json(DeviceResponse {
                id: device.id().clone(),
                physical_id: device.physical_id().to_string(),
                user_id: device.user_id().clone(),
                name: device.name().to_string(),
                event_format: device.event_format().to_string(),
                event_data: event_data,
            }))
        }
        Err(err) => Err(log_and_return_response(err)),
    }
}

fn log_and_return_response(err: DeviceServiceError) -> Response {
    log_device_service_error(&err);
    ErrorResponse::from(err).into_response()
}

pub fn parse_event_data(event_data_raw: &Map<String, Value>) -> Result<Vec<(String, EventDataType)>, String> {
    let mut data = Vec::new();
    for (k, v) in event_data_raw.iter() {
        let edt = match EventDataType::from_str(v.as_str().unwrap_or("")) {
            Ok(e) => e,
            Err(_) => {
                return Err(format!("Invalid event data type for key {}", k));
            }
            
        };
        data.push((k.clone(), edt));
    }
    return Ok(data)
}



pub struct CreateDeviceRequest {
    pub physical_id: String,
    pub user_id: Uuid,
    pub name: String,
    pub event_format: EventFormat,
    pub event_data: HashMap<String, EventDataType>
}

impl TryFrom<Value> for CreateDeviceRequest {
    type Error = String;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let user_id_raw = value.get("user_id");
        let user_id = match user_id_raw {
            Some(id) => id
                .as_str()
                .and_then(|s| Uuid::parse_str(s).ok())
                .ok_or_else(|| String::from("Invalid user_id format"))?,
            None => return Err("Missing user_id".to_string()),
        };
        let physical_id = value
            .get("physical_id")
            .and_then(Value::as_str)
            .map(String::from)
            .ok_or_else(|| "Missing physical_id".to_string())?;
        let name = value
            .get("name")
            .and_then(Value::as_str)
            .map(String::from)
            .ok_or_else(|| "Missing name".to_string())?;
        let event_format = value
            .get("event_format")
            .and_then(Value::as_str)
            .and_then(|s| match s {
                "json" => Some(EventFormat::Json),
                _ => None,
            })
            .ok_or_else(|| "Invalid event_format".to_string())?;
        let event_data_raw = value
            .get("event_data")
            .and_then(Value::as_object);
        let mut event_data = HashMap::new();
        if let Some(data) = event_data_raw {
            event_data = parse_event_data(data)?.into_iter().collect();
        }
        Ok(CreateDeviceRequest {
            user_id,
            physical_id,
            name,
            event_format,
            event_data
        })
    }
}

#[derive(Serialize)]
pub struct DeviceResponse {
    pub id: Uuid,
    pub physical_id: String,
    pub user_id: Uuid,
    pub name: String,
    pub event_format: String,
    pub event_data: HashMap<String, String>,
}