use crate::{application::ports::inbound::{device_service::DeviceServiceError, device_state_service::DeviceStateServiceError, event_service::EventServiceError}, domain::device::EventFormatError};


pub enum HandlerError {
    ParsingError(String),
    ClientError(String),
    InternalError(String),
}

impl From<EventFormatError> for HandlerError {
    fn from(err: EventFormatError) -> Self {
        match err {
            EventFormatError::UnsupportedFormat(e) => HandlerError::ParsingError(format!("Invalid event format : {}", e)),
        }
    }
}

impl From<DeviceServiceError> for HandlerError {
    fn from(value: DeviceServiceError) -> Self {
        match value {
            DeviceServiceError::NotFound => HandlerError::ClientError("Device not found".to_string()),
            DeviceServiceError::AlreadyExists => HandlerError::ClientError("Device already exists".to_string()),
            DeviceServiceError::InvalidInput => HandlerError::ClientError("Invalid input provided".to_string()),
            DeviceServiceError::InternalError(err) => HandlerError::InternalError(format!("on device service : {}", err)),
        }
    }
}

impl From<DeviceStateServiceError> for HandlerError {
    fn from(value: DeviceStateServiceError) -> Self {
        match value {
            DeviceStateServiceError::DeviceNotFound => HandlerError::ClientError("Device not found".to_string()),
            DeviceStateServiceError::DeviceStateNotFound => HandlerError::ClientError("Device state not found".to_string()),
            DeviceStateServiceError::AlreadyExists => HandlerError::ClientError("Device already exists".to_string()),
            DeviceStateServiceError::InvalidInput => HandlerError::ClientError("Invalid input provided".to_string()),
            DeviceStateServiceError::InternalError(err) => HandlerError::InternalError(format!("on device state service, {}", err)),
        }
    }
}

impl From<EventServiceError> for HandlerError {
    fn from(value: EventServiceError) -> Self {
        match value {
            EventServiceError::InvalidInput(s) => HandlerError::ClientError(format!("invalid input, {}", s)),
            EventServiceError::InternalError(err) => HandlerError::InternalError(format!("on event service, {}", err)),
        }
    }
}