use std::sync::Arc;

use axum::{
    extract::{Path, State},
    response::{IntoResponse, Response},
};
use tracing::{instrument, trace, warn};
use uuid::Uuid;

use crate::{
    application::ports::{app::AppOutbound, inbound::device_service::DeviceService},
    infrastructure::http::axum::{
        device_handlers::utils::log_and_return_response, error::ErrorResponse,
    },
};

#[instrument]
pub async fn delete_device_handler<AO: AppOutbound>(
    State(services): State<Arc<AO>>,
    Path(device_id): Path<String>,
) -> Result<(), Response> {
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

    match service.delete_device(id).await {
        Ok(_) => {
            trace!(result = "success");
            Ok(())
        }
        Err(err) => Err(log_and_return_response(err)),
    }
}
