use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::{collections::HashMap, hash::Hash };

use crate::domain::{device::Device, event::{event_data_type::EventDataType, event_data_value::EventDataValue, event_format::EventFormatError}};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: Uuid,
    pub device_physical_id: String,
    pub event_name: String,
    pub timestamp: DateTime<Utc>,
    pub payload: HashMap<String, EventDataValue>,
}

impl Hash for Event {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
        self.device_physical_id.hash(state);
        self.timestamp.hash(state);
    }
}
impl Event {
    pub fn new(device_physical_id: String, event_name: &str, timestamp: &DateTime<Utc>, payload: HashMap<String, EventDataValue>) -> Self{
        return Self {
            id: Uuid::new_v4(),
            device_physical_id,
            event_name: event_name.to_string(),
            timestamp: *timestamp,
            payload,
        };
    }
    pub fn new_checked(device: &Device, timestamp: &DateTime<Utc>, event_name: &str, payload: &[u8]) -> Result<Self, EventFormatError> {
        let event_concerned = match device.events().get(event_name) {
            Some(evt) => evt,
            None => return Err(EventFormatError::UnsupportedFormat(format!("Event '{event_name}' not found in device events")))
        };
        let payload_received = event_concerned.format().decode_event(&payload)?;
        println!("Received payload: {:?}", payload_received);
        // iterate over the device's event_data to ensure all keys in payload are valid
        for (key, data_type) in event_concerned.payload().clone().into_iter() {
            let is_valid = match payload_received.get(&key) {
                Some(EventDataValue::String(_)) => data_type == EventDataType::String,
                Some(EventDataValue::Number(_)) => data_type == EventDataType::Number,
                Some(EventDataValue::Boolean(_)) => data_type == EventDataType::Boolean,
                None => return Err(EventFormatError::UnsupportedFormat(format!("Key '{}' not found in payload", key)))
            };
            if !is_valid {
                return Err(EventFormatError::UnsupportedFormat(format!("Invalid value for key {}, {} expected", &key, &data_type.to_string())));
            }
        }
        return Ok(Self {
            id: Uuid::new_v4(),
            device_physical_id: device.physical_id().to_owned(),
            event_name: event_name.to_string(),
            timestamp: *timestamp,
            payload: payload_received,
        });
    }
}

