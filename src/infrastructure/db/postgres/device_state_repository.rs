use std::collections::HashMap;

use serde_json::Value;
use sqlx::{PgPool, Row};

use crate::{
    application::ports::outbound::device_state_repository::{
        CreateDeviceStateRepository, DeleteDeviceStateRepository, DeviceStateRepositoryError,
        GetDeviceStateRepository, UpdateDeviceStateRepository,
    },
    domain::{event::event_data_value::EventDataValue, state::DeviceState},
};

#[derive(Debug)]
pub struct PostgresDeviceStateRepository {
    pool: PgPool,
}

impl PostgresDeviceStateRepository {
    pub async fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn init(&self) {
        // Ensure the device_states table exists
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS device_states (
            device_id UUID PRIMARY KEY,
            last_update TIMESTAMPTZ NOT NULL,
            values JSONB NOT NULL
        )",
        )
        .execute(&self.pool)
        .await
        .expect("Failed to create device_states table");
    }
}

impl CreateDeviceStateRepository for PostgresDeviceStateRepository {
    async fn create(&self, device_state: &DeviceState) -> Result<(), DeviceStateRepositoryError> {
        let query = "INSERT INTO device_states (device_id, last_update, values) VALUES ($1, $2, $3)
                     ON CONFLICT (device_id) DO UPDATE SET last_update = $2, values = $3";

        let values: HashMap<String, Value> = device_state
            .values
            .clone()
            .into_iter()
            .map(|(k, v)| (k, v.into()))
            .collect();
        sqlx::query(query)
            .bind(sqlx::types::Uuid::from(device_state.device_id))
            .bind(chrono::DateTime::<chrono::Utc>::from(
                device_state.last_update,
            ))
            .bind(sqlx::types::Json::from(values))
            .execute(&self.pool)
            .await
            .map_err(|e: sqlx::Error| {
                println!("Error saving device state: {:?}", e);
                DeviceStateRepositoryError::InternalError(e.to_string())
            })?;
        Ok(())
    }
}

impl GetDeviceStateRepository for PostgresDeviceStateRepository {
    async fn get_by_id(
        &self,
        id: uuid::Uuid,
    ) -> Result<Option<DeviceState>, DeviceStateRepositoryError> {
        let query = "SELECT device_id, last_update, values FROM device_states WHERE device_id = $1";
        let row = sqlx::query(query)
            .bind(sqlx::types::Uuid::from(id))
            .fetch_optional(&self.pool)
            .await
            .map_err(|e: sqlx::Error| {
                println!("Error fetching device state by ID {}: {:?}", id, e);
                DeviceStateRepositoryError::InternalError(e.to_string())
            })?;

        match row {
            Some(row) => {
                let device_id: uuid::Uuid = row.get("device_id");
                let last_update: chrono::DateTime<chrono::Utc> = row.get("last_update");
                let values_db: sqlx::types::Json<HashMap<String, Value>> = row.get("values");
                let mut values = HashMap::new();
                for (k, v) in values_db.0 {
                    let val = EventDataValue::try_from(v).map_err(|_| {
                        DeviceStateRepositoryError::InternalError(format!(
                            "Invalid data stored for key {}",
                            k
                        ))
                    })?;
                    values.insert(k, val);
                }
                Ok(Some(DeviceState {
                    device_id,
                    last_update: last_update.into(),
                    values,
                }))
            }
            None => Ok(None),
        }
    }
}

impl DeleteDeviceStateRepository for PostgresDeviceStateRepository {
    async fn delete_by_id(&self, id: uuid::Uuid) -> Result<(), DeviceStateRepositoryError> {
        let query = "DELETE FROM device_states WHERE device_id = $1";
        sqlx::query(query)
            .bind(sqlx::types::Uuid::from(id))
            .execute(&self.pool)
            .await
            .map_err(|e: sqlx::Error| {
                println!("Error deleting device state by ID {}: {:?}", id, e);
                DeviceStateRepositoryError::InternalError(e.to_string())
            })?;
        Ok(())
    }
}

impl UpdateDeviceStateRepository for PostgresDeviceStateRepository {
    async fn update(&self, device_state: &DeviceState) -> Result<(), DeviceStateRepositoryError> {
        self.create(device_state).await
    }
}
