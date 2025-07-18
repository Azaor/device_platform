use axum::response::{IntoResponse, Response};
use serde::Serialize;

use crate::{application::ports::inbound::{device_service::DeviceServiceError, device_state_service::DeviceStateServiceError, event_service::EventServiceError}, domain::device::EventFormatError};

#[derive(Serialize)]
pub struct ErrorResponse {
    pub status: u16,
    pub message: String,
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        let status = axum::http::StatusCode::from_u16(self.status).unwrap_or(axum::http::StatusCode::INTERNAL_SERVER_ERROR);
        let body = axum::Json(self);
        (status, body).into_response()
    }
}

impl From<ErrorResponse> for Response {
    fn from(err: ErrorResponse) -> Self {
        err.into_response()
    }
}

impl From<DeviceServiceError> for ErrorResponse {
    fn from(err: DeviceServiceError) -> Self {
        match err {
            DeviceServiceError::NotFound => ErrorResponse {
                status: 404,
                message: "Device not found".to_string(),
            },
            DeviceServiceError::AlreadyExists => ErrorResponse {
                status: 409,
                message: "Device already exists".to_string(),
            },
            DeviceServiceError::InvalidInput => ErrorResponse {
                status: 400,
                message: "Invalid input".to_string(),
            },
            DeviceServiceError::InternalError => ErrorResponse {
                status: 500,
                message: "Internal server error".to_string(),
            },
        }
    }
}

impl From<DeviceStateServiceError> for ErrorResponse {
    fn from(err: DeviceStateServiceError) -> Self {
        match err {
            DeviceStateServiceError::DeviceNotFound => ErrorResponse {
                status: 404,
                message: "Device not found".to_string(),
            },
            DeviceStateServiceError::DeviceStateNotFound => ErrorResponse {
                status: 404,
                message: "Device state not found".to_string(),
            },
            DeviceStateServiceError::AlreadyExists => ErrorResponse {
                status: 409,
                message: "Device state already exists".to_string(),
            },
            DeviceStateServiceError::InternalError => ErrorResponse {
                status: 500,
                message: "Internal server error".to_string(),
            },
            DeviceStateServiceError::InvalidInput => ErrorResponse {
                status: 400,
                message: "Invalid input".to_string(),
            },
        }
    }
}

impl From<EventServiceError> for ErrorResponse {
    fn from(err: EventServiceError) -> Self {
        match err {
            EventServiceError::InternalError(val) => ErrorResponse {
                status: 404,
                message: format!("Event not found: {}", val.to_string()),
            },
            EventServiceError::InvalidInput(val) => ErrorResponse {
                status: 409,
                message: format!("Invalid input: {}", val.to_string()),
            },
        }
    }
}

impl From<EventFormatError> for ErrorResponse {
    fn from(err: EventFormatError) -> Self {
        match err {
            EventFormatError::UnsupportedFormat(e) => ErrorResponse {
                status: 400,
                message: format!("Invalid event format: {}", e),
            },
        }
    }
}