use std::{collections::HashMap, sync::Arc};

use axum::{extract::{Path, State}, response::{IntoResponse, Response}, Json};
use chrono::{DateTime, Utc};
use serde::Serialize;
use serde_json::Value;
use tracing::{instrument, trace, warn};
use uuid::Uuid;

use crate::{
    application::ports::{
        app::AppOutbound, inbound::device_state_service::{DeviceStateService, DeviceStateServiceError}
    }, domain::state::DeviceState, infrastructure::{http::axum::error::ErrorResponse, utils::log_device_state_service_error}
};

#[instrument]
pub async fn get_device_state_handler<AO: AppOutbound>(
    State(app_state): State<Arc<AO>>,
    Path(device_id) : Path<String>,
) -> Result<Json<DeviceStateResponse>, Response> {
    let service = app_state.get_device_state_service();
    let id = match Uuid::parse_str(&device_id) {
        Ok(id) => id,
        Err(_) => {
            warn!(result = "warn", details = format!("Invalid device ID"));
            return Err(ErrorResponse { status: 400, message: "Invalid device ID".to_string() }.into_response());
        },
    };

    match service.get_device_state(id).await {
        Ok(Some(device_state)) => {
            trace!(result = "success");
            Ok(Json(DeviceStateResponse::from(device_state)))
        },
        Ok(None) => {
            warn!(result = "warn", details = format!("Device state with ID {} not found", device_id));
            Err(ErrorResponse { status: 404, message: "Device state not found".to_string() }.into_response())
        },
        Err(err) => {
            Err(log_and_return_response(err))
        },
    }
}

pub(crate) fn log_and_return_response(err: DeviceStateServiceError) -> Response {
    log_device_state_service_error(&err);
    ErrorResponse::from(err).into_response()
}

#[derive(Serialize)]
pub struct DeviceStateResponse {
    pub device_id: Uuid,
    pub last_update: DateTime<Utc>,
    pub values: HashMap<String, Value>,
}

impl From<DeviceState> for DeviceStateResponse {
    fn from(state: DeviceState) -> Self {
        let values = state.values.into_iter().map(|(k, v)| (k, v.into())).collect();
        DeviceStateResponse {
            device_id: state.device_id,
            last_update: state.last_update,
            values,
        }
    }
}