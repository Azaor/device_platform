use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::domain::device::{EventDataType, EventEmittable, EventFormat};

pub fn serialize_event_data(event_data: &HashMap<String, EventEmittable>) -> HashMap<String, EventEmittableDb> {
    event_data.iter().map(|(k, v)| {
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