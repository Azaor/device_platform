use std::{collections::HashMap, fmt::Display};

use serde_json::Value;

use crate::domain::event::event_data_value::EventDataValue;

#[derive(Debug, Clone)]
pub enum EventFormat {
    Json,
}

impl EventFormat {
    pub fn decode_event(
        &self,
        payload: &[u8],
    ) -> Result<HashMap<String, EventDataValue>, EventFormatError> {
        match self {
            EventFormat::Json => {
                // For simplicity, we assume the payload is a JSON string that can be parsed into a HashMap
                // In a real application, you would use a JSON library to parse the payload
                let json_str = String::from_utf8_lossy(payload);
                let mut payload = HashMap::new();
                let event_raw: HashMap<String, Value> = serde_json::from_str(&json_str)
                    .map_err(|e| EventFormatError::UnsupportedFormat(e.to_string()))?;
                // iterate over the device's event_data to ensure all keys in payload are valid
                for (key, value) in event_raw.into_iter() {
                    let value = EventDataValue::try_from(value)
                        .map_err(|_| EventFormatError::UnsupportedFormat(key.clone()))?;
                    payload.insert(key, value);
                }
                return Ok(payload);
            }
        }
    }
    pub fn encode_event(
        &self,
        event_payload: HashMap<String, EventDataValue>,
    ) -> Result<String, EventFormatError> {
        match self {
            EventFormat::Json => {
                let payload_converted: HashMap<String, Value> = event_payload
                    .iter()
                    .map(|(k, v)| {
                        let val = match v {
                            EventDataValue::String(s) => {
                                Value::from(s.to_owned())
                            }
                            EventDataValue::Number(n) => {
                                Value::from(n.to_owned())
                            }
                            EventDataValue::Boolean(b) => {
                                Value::from(b.to_owned())
                            }
                        };
                        (k.to_string(), val)
                    })
                    .collect();
                return serde_json::to_string(&payload_converted)
                    .map_err(|e| EventFormatError::UnsupportedFormat(e.to_string()));
            }
        }
    }
}

impl Display for EventFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventFormat::Json => write!(f, "json"),
        }
    }
}

impl TryFrom<&str> for EventFormat {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "json" => Ok(EventFormat::Json),
            _ => Err(format!("Unsupported event format: {}", value)),
        }
    }
}

pub enum EventFormatError {
    UnsupportedFormat(String),
}

impl Display for EventFormatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventFormatError::UnsupportedFormat(msg) => {
                write!(f, "Unsupported event format: {}", msg)
            }
        }
    }
}