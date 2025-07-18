use std::collections::HashMap;

use crate::{domain::device::EventDataType};

pub fn serialize_event_data(event_data: &HashMap<String, EventDataType>) -> HashMap<String, String> {
    event_data.iter().map(|(k, v)| {
        let value = v.to_string();
        (k.clone(), value)
    }).collect()
}