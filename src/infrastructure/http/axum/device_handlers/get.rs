use std::{collections::HashMap, str::FromStr, sync::Arc};

use axum::{
    Json,
    extract::{Path, State},
    response::{IntoResponse, Response},
};
use tracing::{instrument, trace, warn};
use uuid::Uuid;

use crate::{
    application::ports::{app::AppOutbound, inbound::device_service::DeviceService},
    infrastructure::http::axum::{
        device_handlers::{
            types::{DeviceResponse, EventEmittableSerializable},
            utils::log_and_return_response,
        },
        error::ErrorResponse,
    },
};

#[instrument]
pub async fn get_device_handler<AO: AppOutbound>(
    State(services): State<Arc<AO>>,
    Path(device_id): Path<String>,
) -> Result<Json<DeviceResponse>, Response> {
    let service = services.get_device_service();
    let id = match Uuid::parse_str(&device_id) {
        Ok(id) => id,
        Err(_) => {
            warn!(
                result = "warn",
                details = format!("Invalid device id provided : {}", device_id)
            );
            return Err(ErrorResponse {
                status: 400,
                message: "Invalid device ID".to_string(),
            }
            .into_response());
        }
    };

    match service.get_device(id).await {
        Ok(Some(device)) => {
            let events: HashMap<String, EventEmittableSerializable> = device
                .events()
                .clone()
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect();
            trace!(result = "success");
            Ok(Json(DeviceResponse {
                id: device.id().clone(),
                physical_id: device.physical_id().to_string(),
                user_id: device.user_id().clone(),
                name: device.name().to_string(),
                events,
            }))
        }
        Ok(None) => {
            warn!(
                result = "warn",
                details = format!("Device with ID {} not found in DB", device_id)
            );
            Err(ErrorResponse {
                status: 404,
                message: "Device not found".to_string(),
            }
            .into_response())
        }
        Err(err) => Err(log_and_return_response(err)),
    }
}

#[instrument]
pub async fn get_device_by_physical_id<AO: AppOutbound>(
    State(services): State<Arc<AO>>,
    Path(physical_id): Path<String>,
) -> Result<Json<DeviceResponse>, Response> {
    let service = services.get_device_service();

    match service.get_device_by_physical_id(&physical_id).await {
        Ok(Some(device)) => {
            let events: HashMap<String, EventEmittableSerializable> = device
                .events()
                .clone()
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect();
            trace!(result = "success");
            Ok(Json(DeviceResponse {
                id: device.id().clone(),
                physical_id: device.physical_id().to_string(),
                user_id: device.user_id().clone(),
                name: device.name().to_string(),
                events,
            }))
        }
        Ok(None) => {
            warn!(
                result = "warn",
                details = format!("Device with physical ID {} not found in DB", physical_id)
            );
            Err(ErrorResponse {
                status: 404,
                message: "Device not found".to_string(),
            }
            .into_response())
        }
        Err(err) => Err(log_and_return_response(err)),
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
                let events: HashMap<String, EventEmittableSerializable> = device
                    .events()
                    .clone()
                    .into_iter()
                    .map(|(k, v)| (k, v.into()))
                    .collect();
                device_responses.push(DeviceResponse {
                    id: device.id().clone(),
                    physical_id: device.physical_id().to_string(),
                    user_id: device.user_id().clone(),
                    name: device.name().to_string(),
                    events,
                })
            }
            trace!(result = "success");
            Ok(Json(device_responses))
        }
        Err(err) => Err(log_and_return_response(err)),
    }
}
