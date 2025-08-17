use std::{sync::Arc, usize};

use axum::{
    Json, body,
    extract::{Path, Request, State},
    response::{IntoResponse, Response},
};
use chrono::Utc;
use tracing::{instrument, trace, warn};

use crate::{
    application::ports::{
        app::AppOutbound,
        inbound::{
            action_service::ActionService,
            device_service::DeviceService,
        },
    },
    domain::{action::action::Action},
    infrastructure::http::axum::{
        action_handlers::types::{ActionResponse, log_and_return_response},
        error::ErrorResponse,
    },
};

#[instrument]
pub async fn create_action_handler<AO: AppOutbound>(
    State(services): State<Arc<AO>>,
    Path((device_physical_id, action_name)): Path<(String, String)>,
    r: Request,
) -> Result<Json<ActionResponse>, Response> {
    let request_body = r.into_body();
    let body_bytes = match body::to_bytes(request_body, usize::MAX).await {
        Ok(body_bytes) => body_bytes,
        Err(e) => {
            warn!(result = "warn", details = %e);
            return Err(ErrorResponse {
                status: 400,
                message: "Invalid body provided".to_string(),
            }
            .into_response());
        }
    };
    let device_service = services.get_device_service();
    let action_service = services.get_action_service();
    let device = match device_service
        .get_device_by_physical_id(&device_physical_id)
        .await
    {
        Ok(Some(device)) => device,
        Ok(None) => {
            warn!(
                result = "warn",
                details = format!("Device with ID {} not found in DB", &device_physical_id)
            );
            return Err(ErrorResponse {
                status: 404,
                message: "Device not found".to_string(),
            }
            .into_response());
        }
        Err(err) => return Err(ErrorResponse::from(err).into_response()),
    };

    let action = match Action::new_checked(&device, &Utc::now(), &action_name, &body_bytes) {
        Ok(event) => event,
        Err(err) => {
            warn!(result = "warn", details = %err);
            return Err(ErrorResponse::from(err).into_response());
        }
    };
    let action_concerned = match device.actions().get(&action_name) {
        Some(action) => action,
        None => return Err(ErrorResponse::internal_error().into_response()),
    };
    let res = match action_service
        .send_action(action.clone(), &action_concerned.format())
        .await
    {
        Ok(_) => Json(ActionResponse::from(action.clone())),
        Err(err) => return Err(log_and_return_response(err)),
    };
    trace!(result = "success");
    Ok(res)
}
