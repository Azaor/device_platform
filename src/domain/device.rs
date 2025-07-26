
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;
use std::fmt::Display;
use std::{collections::HashMap};

use crate::domain::event::EventDataValue;

#[derive(Debug, Clone)]
pub struct Device {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub event_format: EventFormat,
    pub event_data: HashMap<String, EventDataType>,
}

impl Device {
    pub fn new(id: &Uuid, user_id: &Uuid, name: &str, event_format: EventFormat, event_data: HashMap<String, EventDataType>) -> Self {
        return Self { id: id.clone(), user_id: user_id.clone(), name: name.to_string(), event_format, event_data }
    }
}

#[derive(Debug, Clone)]
pub enum EventFormat {
    Json,
}

impl EventFormat {
    pub fn decode_event(&self, payload: &[u8]) -> Result<HashMap<String, String>, EventFormatError> {
        match self {
            EventFormat::Json => {
                // For simplicity, we assume the payload is a JSON string that can be parsed into a HashMap
                // In a real application, you would use a JSON library to parse the payload
                let json_str = String::from_utf8_lossy(payload);
                return serde_json::from_str(&json_str)
                    .map_err(|e| EventFormatError::UnsupportedFormat(e.to_string()));
            },
        }
    } 
    pub fn encode_event(&self, event_payload: HashMap<String, EventDataValue>) -> Result<String, EventFormatError> {
        match self {
            EventFormat::Json => {
                let payload_converted: HashMap<String, Value> = event_payload.iter().map(|(k ,v)| {
                    let val = match v {
                        crate::domain::event::EventDataValue::String(s) => Value::from(s.to_owned()),
                        crate::domain::event::EventDataValue::Number(n) => Value::from(n.to_owned()),
                        crate::domain::event::EventDataValue::Boolean(b) => Value::from(b.to_owned()),
                    };
                    (k.to_string(), val)
                }).collect();
                return serde_json::to_string(&payload_converted).map_err(|e| EventFormatError::UnsupportedFormat(e.to_string()));
            },
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

#[derive(Debug, Clone)]
pub enum EventDataType {
    String,
    Number,
    Boolean,
}

impl EventDataType {
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "string" => Ok(EventDataType::String),
            "number" => Ok(EventDataType::Number),
            "boolean" => Ok(EventDataType::Boolean),
            _ => Err(format!("Unsupported event data type: {}", s)),
        }
    }
}

impl Display for EventDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventDataType::String => write!(f, "string"),
            EventDataType::Number => write!(f, "number"),
            EventDataType::Boolean => write!(f, "boolean"),
        }
    }
}

impl Serialize for EventDataType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for EventDataType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        let s = String::deserialize(deserializer)?;
        EventDataType::from_str(&s).map_err(serde::de::Error::custom)
    }
}

