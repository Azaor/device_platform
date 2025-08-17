use std::fmt::Display;

use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Eq)]
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
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for EventDataType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        EventDataType::from_str(&s).map_err(serde::de::Error::custom)
    }
}
