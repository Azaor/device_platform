use std::{collections::HashMap};

use crate::domain::event::{event_data_type::EventDataType, event_format::EventFormat};

#[derive(Debug, Clone)]
pub struct EventEmittable {
    format: EventFormat,
    payload: HashMap<String, EventDataType>,
}

impl EventEmittable {
    pub fn new(format: EventFormat, payload: HashMap<String, EventDataType>) -> Self {
        Self { format, payload }
    }
    pub fn format(&self) -> &EventFormat {
        &self.format
    }
    pub fn payload(&self) -> &HashMap<String, EventDataType> {
        &self.payload
    }
}
