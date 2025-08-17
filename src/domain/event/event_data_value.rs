
use serde_json::Value;

use crate::domain::event::event_data_type::EventDataType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventDataValue {
    String(String),
    Number(u64),
    Boolean(bool),
}

impl EventDataValue {
    pub fn parse_event_data_type(data_type: EventDataType, value: &str) -> Result<EventDataValue, EventDataValueError> {
        match data_type {
            EventDataType::String => Ok(EventDataValue::String(value.to_string())),
            EventDataType::Number => {
                let num = value.parse::<u64>().map_err(|_| EventDataValueError::InvalidNumber(value.to_owned()))?;
                Ok(EventDataValue::Number(num))
            }
            EventDataType::Boolean => {
                match value.to_lowercase().as_str() {
                    "true" | "1" => Ok(EventDataValue::Boolean(true)),
                    "false" | "0" => Ok(EventDataValue::Boolean(false)),
                    _ => Err(EventDataValueError::InvalidBoolean(value.to_owned())),
                }
            }
        }
    }
}

impl From<EventDataValue> for Value {
    fn from(value: EventDataValue) -> Self {
        match value {
            EventDataValue::String(s) => Value::from(s.to_owned()),
            EventDataValue::Number(n) => Value::from(n.to_owned()),
            EventDataValue::Boolean(b) => Value::from(b.to_owned()),
        }
    }
}

impl TryFrom<Value> for EventDataValue {
    type Error = EventDataValueError;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Bool(b) => Ok(EventDataValue::Boolean(b)),
            Value::Number(n) => Ok(EventDataValue::Number(n.as_u64().expect("Should be a valid u64"))),
            Value::String(s) => Ok(EventDataValue::String(s)),
            _ => return Err(EventDataValueError::InvalidType)
        }
    }
}

impl TryFrom<&str> for EventDataValue {
    type Error = EventDataValueError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(EventDataValueError::InvalidType);
        }
        // Attempt to parse as a number first
        if let Ok(num) = value.parse::<u64>() {
            return Ok(EventDataValue::Number(num));
        }
        // Attempt to parse as a boolean
        if let Ok(boolean) = value.parse::<bool>() {
            return Ok(EventDataValue::Boolean(boolean));
        }
        // Otherwise, treat it as a string
        Ok(EventDataValue::String(value.to_string()))
    }
}

#[derive(Debug)]
pub enum EventDataValueError {
    InvalidType,
    InvalidNumber(String),
    InvalidBoolean(String)
}
