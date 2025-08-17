use serde_json::Value;

use crate::domain::action::action_data_type::ActionDataType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActionDataValue {
    String(String),
    Number(u64),
    Boolean(bool),
}

impl ActionDataValue {
    pub fn parse_action_data_type(
        data_type: ActionDataType,
        value: &str,
    ) -> Result<ActionDataValue, EventDataValueError> {
        match data_type {
            ActionDataType::String => Ok(ActionDataValue::String(value.to_string())),
            ActionDataType::Number => {
                let num = value
                    .parse::<u64>()
                    .map_err(|_| EventDataValueError::InvalidNumber(value.to_owned()))?;
                Ok(ActionDataValue::Number(num))
            }
            ActionDataType::Boolean => match value.to_lowercase().as_str() {
                "true" | "1" => Ok(ActionDataValue::Boolean(true)),
                "false" | "0" => Ok(ActionDataValue::Boolean(false)),
                _ => Err(EventDataValueError::InvalidBoolean(value.to_owned())),
            },
        }
    }
}

impl From<ActionDataValue> for Value {
    fn from(value: ActionDataValue) -> Self {
        match value {
            ActionDataValue::String(s) => Value::from(s.to_owned()),
            ActionDataValue::Number(n) => Value::from(n.to_owned()),
            ActionDataValue::Boolean(b) => Value::from(b.to_owned()),
        }
    }
}

impl TryFrom<Value> for ActionDataValue {
    type Error = EventDataValueError;
    fn try_from(value: Value) -> Result<Self, Self::Error> {
        match value {
            Value::Bool(b) => Ok(ActionDataValue::Boolean(b)),
            Value::Number(n) => Ok(ActionDataValue::Number(
                n.as_u64().expect("Should be a valid u64"),
            )),
            Value::String(s) => Ok(ActionDataValue::String(s)),
            _ => return Err(EventDataValueError::InvalidType),
        }
    }
}

impl TryFrom<&str> for ActionDataValue {
    type Error = EventDataValueError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(EventDataValueError::InvalidType);
        }
        // Attempt to parse as a number first
        if let Ok(num) = value.parse::<u64>() {
            return Ok(ActionDataValue::Number(num));
        }
        // Attempt to parse as a boolean
        if let Ok(boolean) = value.parse::<bool>() {
            return Ok(ActionDataValue::Boolean(boolean));
        }
        // Otherwise, treat it as a string
        Ok(ActionDataValue::String(value.to_string()))
    }
}

#[derive(Debug)]
pub enum EventDataValueError {
    InvalidType,
    InvalidNumber(String),
    InvalidBoolean(String),
}
