use serde::Serialize;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize)]
pub struct DeviceState {
    pub device_id: Uuid,
    pub last_update: DateTime<Utc>,
    pub values: HashMap<String, String>,
}
