use std::collections::HashMap;

use axum::response::{IntoResponse, Response};
use tracing::warn;

use crate::{
    application::ports::inbound::device_service::DeviceServiceError, domain::{action::{action_data_type::ActionDataType, action_emittable::ActionEmittable, action_format::ActionFormat}, event::{event_data_type::EventDataType, event_emittable::EventEmittable, event_format::EventFormat}}, infrastructure::{
        http::axum::{device_handlers::types::{ActionEmittableSerializable, EventEmittableSerializable}, error::ErrorResponse},
        utils::log_device_service_error,
    }
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


pub fn into_action_emittable(
    payload: HashMap<String, ActionEmittableSerializable>,
) -> Result<HashMap<String, ActionEmittable>, Response> {
    let mut actions = HashMap::new();
    for (k, v) in payload.into_iter() {
        let format = match ActionFormat::try_from(v.format.as_str()) {
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
            let data_type = match ActionDataType::from_str(data_type_raw.as_str()) {
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
        actions.insert(k, ActionEmittable::new(format, payload));
    }
    return Ok(actions);
}
