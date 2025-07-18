use std::{collections::HashMap, sync::Arc};

use axum::{
    Json,
    extract::{Path, State},
    response::{IntoResponse, Response},
};
use serde::Serialize;
use serde_json::Value;
use uuid::Uuid;

use crate::{
    application::ports::{app::AppState, inbound::device_service::DeviceService},
    domain::device::EventFormat,
    infrastructure::http::axum::error::ErrorResponse,
};

pub struct CreateDeviceRequest {
    pub user_id: Uuid,
    pub name: String,
    pub event_format: EventFormat,
}

#[derive(Serialize)]
pub struct DeviceResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub event_data: HashMap<String, String>,
}

impl TryFrom<Value> for CreateDeviceRequest {
    type Error = ErrorResponse;

    fn try_from(value: Value) -> Result<Self, Self::Error> {
        let user_id_raw = value.get("user_id");
        let user_id = match user_id_raw {
            Some(id) => id
                .as_str()
                .and_then(|s| Uuid::parse_str(s).ok())
                .ok_or_else(|| ErrorResponse {
                    status: 400,
                    message: "Invalid user_id".to_string(),
                }),
            None => Err(ErrorResponse {
                status: 400,
                message: "Missing user_id".to_string(),
            }),
        }?;
        let name = value
            .get("name")
            .and_then(Value::as_str)
            .map(String::from)
            .ok_or_else(|| ErrorResponse {
                status: 400,
                message: "Missing name".to_string(),
            })?;
        let event_format = value
            .get("event_format")
            .and_then(Value::as_str)
            .and_then(|s| match s {
                "json" => Some(EventFormat::Json),
                _ => None,
            })
            .ok_or_else(|| ErrorResponse {
                status: 400,
                message: "Invalid event_format".to_string(),
            })?;
        Ok(CreateDeviceRequest {
            user_id,
            name,
            event_format,
        })
    }
}

pub async fn create_device_handler<AS: AppState>(
    State(services): State<Arc<AS>>,
    Json(payload): Json<Value>,
) -> Result<Json<DeviceResponse>, Response> {
    let service = services.get_device_service();
    let payload: CreateDeviceRequest = match CreateDeviceRequest::try_from(payload) {
        Ok(req) => req,
        Err(err) => return Err(err.into_response()),
    };
    match service
        .create_device(payload.user_id, payload.name.clone(), payload.event_format)
        .await
    {
        Ok(device) => {
            let event_data: HashMap<String, String> = device
                .event_data
                .into_iter()
                .map(|(k, v)| (k, v.to_string()))
                .collect();
            Ok(Json(DeviceResponse {
                id: device.id,
                user_id: device.user_id,
                name: device.name,
                event_data: event_data,
            }))
        },
        Err(err) => Err(ErrorResponse::from(err).into_response()),
    }
}

pub async fn get_device_handler<AS: AppState>(
    State(services): State<Arc<AS>>,
    Path(device_id): Path<String>,
) -> Result<Json<DeviceResponse>, Response> {
    let service = services.get_device_service();
    let id = match Uuid::parse_str(&device_id) {
        Ok(id) => id,
        Err(_) => {
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
                .event_data
                .into_iter()
                .map(|(k, v)| (k, v.to_string()))
                .collect();
            Ok(Json(DeviceResponse {
                id: device.id,
                user_id: device.user_id,
                name: device.name,
                event_data: event_data,
            }))
        },
        Ok(None) => Err(ErrorResponse {
            status: 404,
            message: "Device not found".to_string(),
        }
        .into_response()),
        Err(err) => Err(ErrorResponse::from(err).into_response()),
    }
}

pub async fn delete_device_handler<AS: AppState>(
    State(services): State<Arc<AS>>,
    Path(device_id): Path<String>,
) -> Result<(), Response> {
    let service = services.get_device_service();
    let id = match Uuid::parse_str(&device_id) {
        Ok(id) => id,
        Err(_) => {
            return Err(ErrorResponse {
                status: 400,
                message: "Invalid device ID".to_string(),
            }
            .into_response());
        }
    };

    match service.delete_device(id).await {
        Ok(_) => Ok(()),
        Err(err) => Err(ErrorResponse::from(err).into_response()),
    }
}

pub async fn update_device_handler<AS: AppState>(
    State(services): State<Arc<AS>>,
    Path(device_id): Path<String>,
    Json(payload): Json<Value>,
) -> Result<Json<DeviceResponse>, Response> {
    let service = services.get_device_service();
    let id = match Uuid::parse_str(&device_id) {
        Ok(id) => id,
        Err(_) => {
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
    let event_data = payload
        .get("event_data")
        .and_then(Value::as_object)
        .map(|m| {
            m.iter()
                .map(|(k, v)| (k.clone(), v.as_str().unwrap_or("").to_string()))
                .collect()
        });

    match service.update_device(id, name, event_data).await {
        // convert event_data to HashMap<String, String>
        Ok(device) => {
            let event_data: HashMap<String, String> = device
                .event_data
                .into_iter()
                .map(|(k, v)| (k, v.to_string()))
                .collect();
            Ok(Json(DeviceResponse {
                id: device.id,
                user_id: device.user_id,
                name: device.name,
                event_data: event_data,
            }))
        }
        Err(err) => Err(ErrorResponse::from(err).into_response()),
    }
}
