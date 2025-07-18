use std::collections::HashMap;

use sqlx::{PgPool, Row};

use crate::{application::ports::outbound::device_state_repository::{DeviceStateRepository, DeviceStateRepositoryError}, domain::state::DeviceState};

pub struct PostgresDeviceStateRepository {
    pool: PgPool,
}

impl PostgresDeviceStateRepository {
    pub async fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn init(&self) {
        // Ensure the device_states table exists
        sqlx::query("CREATE TABLE IF NOT EXISTS device_states (
            device_id UUID PRIMARY KEY,
            last_update TIMESTAMPTZ NOT NULL,
            values JSONB NOT NULL
        )")
        .execute(&self.pool)
        .await
        .expect("Failed to create device_states table");
    }
}

impl DeviceStateRepository for PostgresDeviceStateRepository {
    async fn save(&self, device_state: &DeviceState) -> Result<(), DeviceStateRepositoryError> {
        let query = "INSERT INTO device_states (device_id, last_update, values) VALUES ($1, $2, $3)
                     ON CONFLICT (device_id) DO UPDATE SET last_update = $2, values = $3";
        
        sqlx::query(query)
            .bind(sqlx::types::Uuid::from(device_state.device_id))
            .bind(chrono::DateTime::<chrono::Utc>::from(device_state.last_update))
            .bind(sqlx::types::Json::from(device_state.values.clone()))
            .execute(&self.pool)
            .await
            .map_err(|e: sqlx::Error| {
                println!("Error saving device state: {:?}", e);
                DeviceStateRepositoryError::InternalError
            })?;
        Ok(())
    }

    async fn find_by_id(&self, uid: uuid::Uuid) -> Result<Option<DeviceState>, DeviceStateRepositoryError> {
        let query = "SELECT device_id, last_update, values FROM device_states WHERE device_id = $1";
        let row = sqlx::query(query)
            .bind(sqlx::types::Uuid::from(uid))
            .fetch_optional(&self.pool)
            .await
            .map_err(|e: sqlx::Error| {
                println!("Error saving device state: {:?}", e);
                DeviceStateRepositoryError::InternalError
            })?;
        match row {
            Some(row) => {
                let device_id: uuid::Uuid = row.get("device_id");
                let last_update: chrono::DateTime<chrono::Utc> = row.get("last_update");
                let values: sqlx::types::Json<HashMap<String, String>> = row.get("values");
                Ok(Some(DeviceState {
                    device_id,
                    last_update: last_update.into(),
                    values: values.0,
                }))
            }
            None => Ok(None),
        }
    }

    async fn delete_by_id(&self, id: uuid::Uuid) -> Result<(), DeviceStateRepositoryError> {
        let query = "DELETE FROM device_states WHERE device_id = $1";
        sqlx::query(query)
            .bind(sqlx::types::Uuid::from(id))
            .execute(&self.pool)
            .await
            .map_err(|e: sqlx::Error| {
                println!("Error saving device state: {:?}", e);
                DeviceStateRepositoryError::InternalError
            })?;
        Ok(())
    }
}