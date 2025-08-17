use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    response::{IntoResponse, Response},
};
use tracing::{instrument, trace, warn};
use uuid::Uuid;

use crate::{
    application::ports::{
        app::AppOutbound,
        inbound::{
            action_service::ActionService,
            device_service::DeviceService,
        },
    },
    infrastructure::http::axum::{
        action_handlers::types::{log_and_return_response, ActionResponse}, error::ErrorResponse,
    },
};

#[instrument]
pub async fn get_actions_handler<AO: AppOutbound>(
    State(services): State<Arc<AO>>,
    Path(device_id): Path<String>,
) -> Result<Json<Vec<ActionResponse>>, Response> {
    let action_service = services.get_action_service();
    let device_service = services.get_device_service();
    let device_id = match Uuid::parse_str(&device_id) {
        Ok(id) => id,
        Err(err) => {
            warn!(result = "warn", details = %err);
            return Err(ErrorResponse {
                status: 400,
                message: "Invalid device_id format".to_string(),
            }
            .into_response());
        }
    };
    let device = match device_service.get_device(device_id).await {
        Ok(Some(device)) => device,
        Ok(None) => {
            warn!(
                result = "warn",
                details = format!("Device with ID {} not found in DB", device_id)
            );
            return Err(ErrorResponse {
                status: 404,
                message: "Device not found".to_string(),
            }
            .into_response());
        }
        Err(err) => return Err(ErrorResponse::from(err).into_response()),
    };
    match action_service.get_actions(&device.id().to_string()).await {
        Ok(actions) => {
            let response: Vec<ActionResponse> =
                actions.into_iter().map(ActionResponse::from).collect();
            trace!(result = "success");
            Ok(Json(response))
        }
        Err(err) => Err(log_and_return_response(err)),
    }
}
