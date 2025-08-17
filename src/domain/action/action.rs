use std::collections::HashMap;

use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::hash::Hash;

use crate::domain::{action::{action_data_type::ActionDataType, action_data_value::ActionDataValue, action_format::ActionFormatError}, device::Device};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Action {
    pub id: Uuid,
    pub device_id: String,
    pub action_name: String,
    pub timestamp: DateTime<Utc>,
    pub payload: HashMap<String, ActionDataValue>,
}

impl Hash for Action {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.action_name.hash(state);
        self.timestamp.hash(state);
    }
}

impl Action {
    pub fn new(device_id: String, action_name: &str, timestamp: &DateTime<Utc>, payload: HashMap<String, ActionDataValue>) -> Self{
        return Self {
            id: Uuid::new_v4(),
            device_id,
            action_name: action_name.to_string(),
            timestamp: *timestamp,
            payload,
        };
    }
    pub fn new_checked(device: &Device, timestamp: &DateTime<Utc>, action_name: &str, payload: &[u8]) -> Result<Self, ActionFormatError> {
        let action_concerned = match device.actions().get(action_name) {
            Some(evt) => evt,
            None => return Err(ActionFormatError::UnsupportedFormat(format!("Action '{action_name}' not found in device events")))
        };
        let payload_received = action_concerned.format().decode_action(&payload)?;
        // iterate over the device's event_data to ensure all keys in payload are valid
        for (key, data_type) in action_concerned.payload().clone().into_iter() {
            let is_valid = match payload_received.get(&key) {
                Some(ActionDataValue::String(_)) => data_type == ActionDataType::String,
                Some(ActionDataValue::Number(_)) => data_type == ActionDataType::Number,
                Some(ActionDataValue::Boolean(_)) => data_type == ActionDataType::Boolean,
                None => return Err(ActionFormatError::UnsupportedFormat(format!("Key '{}' not found in payload", key)))
            };
            if !is_valid {
                return Err(ActionFormatError::UnsupportedFormat(format!("Invalid value for key {}, {} expected", &key, &data_type.to_string())));
            }
        }
        return Ok(Self {
            id: Uuid::new_v4(),
            device_id: device.id().to_string(),
            action_name: action_name.to_string(),
            timestamp: *timestamp,
            payload: payload_received,
        });
    }
}

