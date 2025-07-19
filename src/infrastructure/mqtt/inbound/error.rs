use crate::domain::device::EventFormatError;


pub enum HandlerError {
    ParsingError(String),
    InternalError(String),
}

impl From<EventFormatError> for HandlerError {
    fn from(err: EventFormatError) -> Self {
        match err {
            EventFormatError::UnsupportedFormat(e) => HandlerError::ParsingError(format!("Invalid event format : {}", e)),
        }
    }
}