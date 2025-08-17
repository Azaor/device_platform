use std::collections::HashMap;

use axum::response::{IntoResponse, Response};
use chrono::{DateTime, Utc};
use serde::Serialize;
use serde_json::Value;
use tracing::{error, warn};
use uuid::Uuid;

use crate::application::ports::inbound::{action_service::ActionServiceError};
use crate::domain::action::action::Action;
use crate::infrastructure::http::axum::error::ErrorResponse;




pub fn log_and_return_response(err: ActionServiceError) -> Response {
    match &err {
        ActionServiceError::InvalidInput(err) => warn!(result = "warn", details = format!("invalid input: {}", err)),
        ActionServiceError::InternalError(err) => error!(result = "error", details = format!("internal error: {}", err)),
    };
    return ErrorResponse::from(err).into_response();
}

#[derive(Serialize)]
pub struct ActionResponse {
    pub id: Uuid,
    pub device_id: String,
    pub action_name: String,
    pub timestamp: DateTime<Utc>,
    pub payload: HashMap<String, Value>,
}

impl From<Action> for ActionResponse {
    fn from(event: Action) -> Self {
        let payload = event.payload.into_iter().map(|(k, v)| (k, v.into())).collect();
        ActionResponse {
            id: event.id,
            device_id: event.device_id.to_string(),
            action_name: event.action_name,
            timestamp: event.timestamp,
            payload,
        }
    }
}