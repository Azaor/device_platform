use std::{collections::HashMap, sync::Arc};

use axum::{
    Json,
    extract::State,
    response::{IntoResponse, Response},
};
use serde_json::Value;
use tracing::{error, instrument, trace};
use uuid::Uuid;

use crate::{
    application::ports::{app::AppOutbound, inbound::device_service::DeviceService},
    domain::device::Device,
    infrastructure::http::axum::{
        device_handlers::{
            types::{CreateDeviceRequest, DeviceResponse, EventEmittableSerializable},
            utils::{into_action_emittable, into_event_emittable, log_and_return_response},
        },
        error::ErrorResponse,
    },
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
            }
            .into_response());
        }
    };
    let events = into_event_emittable(payload.events)?;
    let actions = into_action_emittable(payload.actions)?;

    let device = Device::new(
        &Uuid::new_v4(),
        &payload.physical_id,
        &payload.user_id,
        &payload.name.clone(),
        events,
        actions
    );
    match service.create_device(&device).await {
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
