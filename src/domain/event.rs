use serde_json::Value;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::{collections::HashMap, hash::Hash };

use crate::domain::device::{Device, EventDataType, EventFormatError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: Uuid,
    pub device_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub payload: HashMap<String, EventDataValue>,
}

impl Hash for Event {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.device_id.hash(state);
        self.timestamp.hash(state);
    }
}
impl Event {
    pub fn new(device: &Device, timestamp: &DateTime<Utc>, payload: &[u8]) -> Result<Self, EventFormatError> {
        let payload_received = device.event_format.decode_event(&payload)?;
        let mut payload = HashMap::new();
        // iterate over the device's event_data to ensure all keys in payload are valid
        for (key, value) in device.event_data.clone().into_iter() {
            if !payload_received.contains_key(&key) {
                return Err(EventFormatError::UnsupportedFormat(format!("Key '{}' not found in payload", key)));
            }
            let converted_value = match EventDataValue::parse_event_data_type(value, &payload_received[&key]) {
                Ok(v) => v,
                Err(e) => {
                    match e {
                        EventDataValueError::InvalidType => return Err(EventFormatError::UnsupportedFormat(format!("Format given is an invalid one"))),
                        EventDataValueError::InvalidNumber(n) => return Err(EventFormatError::UnsupportedFormat(format!("Value '{}' for key '{}' is not valid, expected a number", n, key))),
                        EventDataValueError::InvalidBoolean(b) => return Err(EventFormatError::UnsupportedFormat(format!("Value '{}' for key '{}' is not valid, expected a boolean", b, key))),
                    }
                }
            };
            payload.insert(key, converted_value);
        }
        let id = Uuid::new_v4();
        let device_id = device.id.clone();
        return Ok(Self {
            id,
            device_id,
            timestamp: *timestamp,
            payload,
        });
    }
}

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

pub enum EventDataValueError {
    InvalidType,
    InvalidNumber(String),
    InvalidBoolean(String)
}

