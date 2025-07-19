use std::{collections::HashMap, sync::Arc};

use axum::{extract::{Path, State}, response::{IntoResponse, Response}, Json};
use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

use crate::{
    application::ports::{
        app::AppOutbound, inbound::device_state_service::DeviceStateService
    },
    infrastructure::http::axum::error::ErrorResponse
};

#[derive(Serialize)]
pub struct DeviceStateResponse {
    pub device_id: Uuid,
    pub last_update: DateTime<Utc>,
    pub values: HashMap<String, String>,
}

pub async fn get_device_state_handler<AO: AppOutbound>(
    State(app_state): State<Arc<AO>>,
    Path(device_id) : Path<String>,
) -> Result<Json<DeviceStateResponse>, Response> {
    let service = app_state.get_device_state_service();
    let id = match Uuid::parse_str(&device_id) {
        Ok(id) => id,
        Err(_) => return Err(ErrorResponse { status: 400, message: "Invalid device ID".to_string() }.into_response()),
    };

    match service.get_device_state(id).await {
        Ok(Some(device_state)) => Ok(Json(DeviceStateResponse {
            device_id: device_state.device_id,
            last_update: device_state.last_update,
            values: device_state.values.into_iter().collect::<HashMap<String, String>>(),
        })),
        Ok(None) => Err(ErrorResponse { status: 404, message: "Device state not found".to_string() }.into_response()),
        Err(err) => {
            Err(ErrorResponse::from(err).into_response())
        },
    }
}
