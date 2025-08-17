use std::collections::HashMap;

use crate::domain::action::{action_data_type::ActionDataType, action_format::ActionFormat};

#[derive(Debug, Clone)]
pub struct ActionEmittable {
    format: ActionFormat,
    payload: HashMap<String, ActionDataType>,
}

impl ActionEmittable {
    pub fn new(format: ActionFormat, payload: HashMap<String, ActionDataType>) -> Self {
        Self { format, payload }
    }

    pub fn format(&self) -> &ActionFormat {
        &self.format
    }

    pub fn payload(&self) -> &HashMap<String, ActionDataType> {
        &self.payload
    }
}
