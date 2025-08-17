use std::{collections::HashMap, sync::Arc};

use axum::{
    Json,
    extract::{Path, State},
    response::{IntoResponse, Response},
};
use serde_json::Value;
use tracing::{error, instrument, trace, warn};
use uuid::Uuid;

use crate::{
    application::ports::{app::AppOutbound, inbound::device_service::DeviceService},
    infrastructure::http::axum::{
        device_handlers::{
            types::{DeviceResponse, EventEmittableSerializable, UpdateDeviceRequest},
            utils::{into_action_emittable, into_event_emittable, log_and_return_response},
        },
        error::ErrorResponse,
    },
};

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

    let payload: UpdateDeviceRequest = match UpdateDeviceRequest::try_from(payload) {
        Ok(req) => req,
        Err(err) => {
            error!(result = "error", details = %err);
            return Err(ErrorResponse {
                status: 400,
                message: err,
            }
            .into_response());
        }
    };
    let events = payload.events.map(into_event_emittable).transpose()?;
    let actions = payload.actions.map(into_action_emittable).transpose()?;
    match service.update_device(id, payload.physical_id, payload.name, events, actions).await {
        // convert event_data to HashMap<String, String>
        Ok(device) => {
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
        Err(err) => Err(log_and_return_response(err)),
    }
}
