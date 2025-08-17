use std::{collections::HashMap, fmt::Display};

use serde_json::Value;

use crate::domain::action::action_data_value::ActionDataValue;

#[derive(Debug, Clone)]
pub enum ActionFormat {
    Json,
}

impl ActionFormat {
    pub fn decode_action(
        &self,
        payload: &[u8],
    ) -> Result<HashMap<String, ActionDataValue>, ActionFormatError> {
        match self {
            ActionFormat::Json => {
                // For simplicity, we assume the payload is a JSON string that can be parsed into a HashMap
                // In a real application, you would use a JSON library to parse the payload
                let json_str = String::from_utf8_lossy(payload);
                let mut payload = HashMap::new();
                let event_raw: HashMap<String, Value> = serde_json::from_str(&json_str)
                    .map_err(|e| ActionFormatError::UnsupportedFormat(e.to_string()))?;
                // iterate over the device's event_data to ensure all keys in payload are valid
                for (key, value) in event_raw.into_iter() {
                    let value = ActionDataValue::try_from(value)
                        .map_err(|_| ActionFormatError::UnsupportedFormat(key.clone()))?;
                    payload.insert(key, value);
                }
                return Ok(payload);
            }
        }
    }
    pub fn encode_event(
        &self,
        event_payload: HashMap<String, ActionDataValue>,
    ) -> Result<String, ActionFormatError> {
        match self {
            ActionFormat::Json => {
                let payload_converted: HashMap<String, Value> = event_payload
                    .iter()
                    .map(|(k, v)| {
                        let val = match v {
                            ActionDataValue::String(s) => Value::from(s.to_owned()),
                            ActionDataValue::Number(n) => Value::from(n.to_owned()),
                            ActionDataValue::Boolean(b) => Value::from(b.to_owned()),
                        };
                        (k.to_string(), val)
                    })
                    .collect();
                return serde_json::to_string(&payload_converted)
                    .map_err(|e| ActionFormatError::UnsupportedFormat(e.to_string()));
            }
        }
    }
}

impl Display for ActionFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActionFormat::Json => write!(f, "json"),
        }
    }
}

impl TryFrom<&str> for ActionFormat {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "json" => Ok(ActionFormat::Json),
            _ => Err(format!("Unsupported event format: {}", value)),
        }
    }
}

pub enum ActionFormatError {
    UnsupportedFormat(String),
}

impl Display for ActionFormatError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActionFormatError::UnsupportedFormat(msg) => {
                write!(f, "Unsupported event format: {}", msg)
            }
        }
    }
}
