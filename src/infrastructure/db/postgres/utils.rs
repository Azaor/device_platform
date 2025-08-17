use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::domain::{action::{action_data_type::ActionDataType, action_emittable::ActionEmittable, action_format::ActionFormat}, event::{event_data_type::EventDataType, event_emittable::EventEmittable, event_format::EventFormat}};

pub fn serialize_event_data(event_data: &HashMap<String, EventEmittable>) -> HashMap<String, EventEmittableDb> {
    event_data.iter().map(|(k, v)| {
        let value = v.into();
        (k.clone(), value)
    }).collect()
}

pub fn serialize_action_data(action_data: &HashMap<String, ActionEmittable>) -> HashMap<String, ActionEmittableDb> {
    action_data.iter().map(|(k, v)| {
        let value = v.into();
        (k.clone(), value)
    }).collect()
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct EventEmittableDb {
    pub format: String,
    pub payload: HashMap<String, String>,
}

impl From<&EventEmittable> for EventEmittableDb {
    fn from(event: &EventEmittable) -> Self {
        Self {
            format: event.format().to_string(),
            payload: event.payload().clone().into_iter().map(|(k, v)| (k, v.to_string())).collect(),
        }
    }
}

impl TryFrom<EventEmittableDb> for EventEmittable {
    type Error = String;

    fn try_from(value: EventEmittableDb) -> Result<Self, Self::Error> {
        let format = EventFormat::try_from(value.format.as_str())?;
        let mut payload = HashMap::new();
        for (key, data_type) in value.payload.iter() {
            let data_type = EventDataType::from_str(&data_type)?;
            payload.insert(key.clone(), data_type);
        }
        Ok(EventEmittable::new(format, payload))
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ActionEmittableDb {
    pub format: String,
    pub payload: HashMap<String, String>,
}

impl From<&ActionEmittable> for ActionEmittableDb {
    fn from(action: &ActionEmittable) -> Self {
        Self {
            format: action.format().to_string(),
            payload: action.payload().clone().into_iter().map(|(k, v)| (k, v.to_string())).collect(),
        }
    }
}

impl TryFrom<ActionEmittableDb> for ActionEmittable {
    type Error = String;

    fn try_from(value: ActionEmittableDb) -> Result<Self, Self::Error> {
        let format = ActionFormat::try_from(value.format.as_str())?;
        let mut payload = HashMap::new();
        for (key, data_type) in value.payload.iter() {
            let data_type = ActionDataType::from_str(&data_type)?;
            payload.insert(key.clone(), data_type);
        }
        Ok(ActionEmittable::new(format, payload))
    }
}