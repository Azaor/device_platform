use std::collections::HashMap;

use axum::response::{IntoResponse, Response};
use tracing::warn;

use crate::{
    application::ports::inbound::device_service::DeviceServiceError,
    domain::device::{EventDataType, EventEmittable, EventFormat},
    infrastructure::{
        http::axum::{device_handlers::types::EventEmittableSerializable, error::ErrorResponse},
        utils::log_device_service_error,
    },
};

pub fn log_and_return_response(err: DeviceServiceError) -> Response {
    log_device_service_error(&err);
    ErrorResponse::from(err).into_response()
}

pub fn into_event_emittable(
    payload: HashMap<String, EventEmittableSerializable>,
) -> Result<HashMap<String, EventEmittable>, Response> {
    let mut events = HashMap::new();
    for (k, v) in payload.into_iter() {
        let format = match EventFormat::try_from(v.format.as_str()) {
            Ok(format) => format,
            Err(_) => {
                warn!(
                    result = "warn",
                    details = format!("Invalid event format for key: {}", k)
                );
                return Err(ErrorResponse {
                    status: 400,
                    message: format!("Invalid event format for key: {}", k),
                }
                .into_response());
            }
        };
        let mut payload = HashMap::new();
        for (data_name, data_type_raw) in v.payload.into_iter() {
            let data_type = match EventDataType::from_str(data_type_raw.as_str()) {
                Ok(dt) => dt,
                Err(_) => {
                    warn!(
                        result = "warn",
                        details = format!(
                            "Invalid event data type for key {} in event {}",
                            data_name, k
                        )
                    );
                    return Err(ErrorResponse {
                        status: 400,
                        message: format!(
                            "Invalid event data type for key {} in event {}",
                            data_name, k
                        ),
                    }
                    .into_response());
                }
            };
            payload.insert(data_name, data_type);
        }
        events.insert(k, EventEmittable::new(format, payload));
    }
    return Ok(events);
}
