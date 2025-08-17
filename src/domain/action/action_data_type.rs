use std::fmt::Display;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActionDataType {
    String,
    Number,
    Boolean,
}

impl ActionDataType {
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "string" => Ok(ActionDataType::String),
            "number" => Ok(ActionDataType::Number),
            "boolean" => Ok(ActionDataType::Boolean),
            _ => Err(format!("Unsupported event data type: {}", s)),
        }
    }
}

impl Display for ActionDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActionDataType::String => write!(f, "string"),
            ActionDataType::Number => write!(f, "number"),
            ActionDataType::Boolean => write!(f, "boolean"),
        }
    }
}

impl Serialize for ActionDataType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for ActionDataType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        ActionDataType::from_str(&s).map_err(serde::de::Error::custom)
    }
}
