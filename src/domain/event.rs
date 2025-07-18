use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::{collections::HashMap, hash::Hash};

use crate::domain::device::{Device, EventFormatError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: Uuid,
    pub device_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub payload: HashMap<String, String>, // simple pour MVP (valeurs numériques)
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
        let payload = device.event_format.decode_event(&payload)?;

        // iterate over the device's event_data to ensure all keys in payload are valid
        for (key, value) in device.event_data.clone().into_iter() {
            if !payload.contains_key(&key) {
                return Err(EventFormatError::UnsupportedFormat(format!("Key '{}' not found in payload", key)));
            }
            if !value.is_valid(&payload[&key]) {
                return Err(EventFormatError::UnsupportedFormat(format!("Value '{}' for key '{}' is not valid", payload[&key], key)));
            }
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