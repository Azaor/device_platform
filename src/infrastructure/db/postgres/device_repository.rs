
use std::collections::HashMap;

use sqlx::{postgres::PgQueryResult, PgPool, Row};
use uuid::Uuid;

use crate::{application::ports::outbound::device_repository::{DeviceRepository, DeviceRepositoryError}, domain::{device::{Device, EventDataType, EventFormat}}, infrastructure::db::postgres::utils::serialize_event_data};

pub struct PostgresDeviceRepository {
    pool: PgPool,
}

impl PostgresDeviceRepository {
    pub async fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    pub async fn init(&self) {
        // Ensure the devices table exists
        sqlx::query("
            CREATE TABLE IF NOT EXISTS devices (
                id UUID PRIMARY KEY,
                user_id UUID NOT NULL,
                name TEXT NOT NULL,
                event_format TEXT NOT NULL,
                event_data JSONB NOT NULL DEFAULT '{}',
                UNIQUE (id, user_id)
            )
        ")
        .execute(&self.pool)
        .await
        .expect("Failed to create devices table");
    }
}

impl DeviceRepository for PostgresDeviceRepository {
    async fn save(&self, device: &Device) -> Result<(), DeviceRepositoryError> {
        let query = "INSERT INTO devices (id, user_id, name, event_format, event_data) VALUES ($1, $2, $3, $4, $5) ON CONFLICT (id) DO UPDATE SET name = $3, event_format = $4, event_data = $5";
        let result: PgQueryResult = sqlx::query(query)
            .bind(sqlx::types::Uuid::from(device.id))
            .bind(sqlx::types::Uuid::from(device.user_id))
            .bind(&device.name)
            .bind(&device.event_format.to_string())
            .bind(sqlx::types::Json::from(serialize_event_data(&device.event_data)))
            .execute(&self.pool)
            .await
            .map_err(|e| {
                println!("Error saving device {}: {:?}", device.id, e);
                DeviceRepositoryError::InternalError
            })?;
        
        if result.rows_affected() == 0 {
            return Err(crate::application::ports::outbound::device_repository::DeviceRepositoryError::Conflict);
        }
        
        Ok(())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Device>, crate::application::ports::outbound::device_repository::DeviceRepositoryError> {
        // Query to find a device by its ID
        let query = "SELECT id, user_id, name, event_format, event_data FROM devices WHERE id = $1";
        let row = sqlx::query(query)
            .bind(sqlx::types::Uuid::from(id))
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| {
                println!("Error fetching device by ID {} : {:?}", id, e);
                DeviceRepositoryError::InternalError
            })?;
        let row = match row {
            Some(row) => row,
            None => return Ok(None), // Device not found
        };
        // Extracting the fields from the row
        let id: Uuid = row.get("id");
        let user_id: Uuid = row.get("user_id");
        let name: String = row.get("name");
        let event_format: String = row.get("event_format");
        let event_data_raw: sqlx::types::Json<HashMap<String, String>> = row.get("event_data");

        let mut event_data = HashMap::new();
        for (key, value) in event_data_raw.0 {
            let event_data_type = EventDataType::from_str(&value)
                .map_err(|_| DeviceRepositoryError::InternalError)?;
            event_data.insert(key, event_data_type);
        }
        // Creating the Device instance
        let device = Device {
            id,
            user_id,
            name,
            event_format: EventFormat::try_from(event_format.as_str()).map_err(|_| DeviceRepositoryError::InternalError)?,
            event_data: event_data,
        };
        Ok(Some(device))
    }

    async fn delete_by_id(&self, id: Uuid) -> Result<(), crate::application::ports::outbound::device_repository::DeviceRepositoryError> {
        let query = "DELETE FROM devices WHERE id = $1";
        let result: PgQueryResult = sqlx::query(query)
            .bind(sqlx::types::Uuid::from(id))
            .execute(&self.pool)
            .await
            .map_err(|_| crate::application::ports::outbound::device_repository::DeviceRepositoryError::InternalError)?;
        
        if result.rows_affected() == 0 {
            return Err(crate::application::ports::outbound::device_repository::DeviceRepositoryError::NotFound);
        }
        
        Ok(())
    }
}

