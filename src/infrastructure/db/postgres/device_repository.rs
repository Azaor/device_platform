use std::collections::HashMap;

use sqlx::{PgPool, Row, postgres::PgQueryResult};
use uuid::Uuid;

use crate::{
    application::ports::outbound::device_repository::{
        CreateDeviceRepository, DeleteDeviceRepository, DeviceRepositoryError, GetDeviceRepository,
        UpdateDeviceRepository,
    },
    domain::device::{Device, EventDataType, EventFormat},
    infrastructure::db::postgres::utils::serialize_event_data,
};

#[derive(Debug)]
pub struct PostgresDeviceRepository {
    pool: PgPool,
}

impl PostgresDeviceRepository {
    pub async fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    pub async fn init(&self) {
        // Ensure the devices table exists
        sqlx::query(
            "
            CREATE TABLE IF NOT EXISTS devices (
                id UUID PRIMARY KEY,
                physical_id TEXT NOT NULL,
                user_id UUID NOT NULL,
                name TEXT NOT NULL,
                event_format TEXT NOT NULL,
                event_data JSONB NOT NULL DEFAULT '{}',
                UNIQUE (id, user_id)
            )
        ",
        )
        .execute(&self.pool)
        .await
        .expect("Failed to create devices table");
    }
}

impl CreateDeviceRepository for PostgresDeviceRepository {
    async fn create(&self, device: &Device) -> Result<(), DeviceRepositoryError> {
        let query = "INSERT INTO devices (id, user_id, name, event_format, event_data) VALUES ($1, $2, $3, $4, $5) ON CONFLICT (id) DO UPDATE SET name = $3, event_format = $4, event_data = $5";
        let result: PgQueryResult = sqlx::query(query)
            .bind(sqlx::types::Uuid::from(*device.id()))
            .bind(sqlx::types::Uuid::from(*device.user_id()))
            .bind(&device.name())
            .bind(&device.event_format().to_string())
            .bind(sqlx::types::Json::from(serialize_event_data(
                &device.event_data(),
            )))
            .execute(&self.pool)
            .await
            .map_err(|e| {
                println!("Error saving device {}: {:?}", device.id(), e);
                DeviceRepositoryError::InternalError(e.to_string())
            })?;

        if result.rows_affected() == 0 {
            return Err(crate::application::ports::outbound::device_repository::DeviceRepositoryError::Conflict);
        }

        Ok(())
    }
}

impl GetDeviceRepository for PostgresDeviceRepository {
    async fn get_by_id(&self, id: Uuid) -> Result<Option<Device>, DeviceRepositoryError> {
        // Query to find a device by its ID
        let query = "SELECT id, user_id, physical_id, name, event_format, event_data FROM devices WHERE id = $1";
        let row = sqlx::query(query)
            .bind(sqlx::types::Uuid::from(id))
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| {
                println!("Error fetching device by ID {} : {:?}", id, e);
                DeviceRepositoryError::InternalError(e.to_string())
            })?;
        let row = match row {
            Some(row) => row,
            None => return Ok(None), // Device not found
        };
        // Extracting the fields from the row
        let id: Uuid = row.get("id");
        let user_id: Uuid = row.get("user_id");
        let physical_id: String = row.get("physical_id");
        let name: String = row.get("name");
        let event_format: String = row.get("event_format");
        let event_data_raw: sqlx::types::Json<HashMap<String, String>> = row.get("event_data");

        let mut event_data = HashMap::new();
        for (key, value) in event_data_raw.0 {
            let event_data_type = EventDataType::from_str(&value)
                .map_err(|e| DeviceRepositoryError::InternalError(e.to_string()))?;
            event_data.insert(key, event_data_type);
        }
        // Creating the Device instance
        let device = Device::new(
            &id,
            &physical_id,
            &user_id,
            &name,
            EventFormat::try_from(event_format.as_str())
                .map_err(|e| DeviceRepositoryError::InternalError(e.to_string()))?,
            event_data,
        );
        Ok(Some(device))
    }

    async fn get_by_user_id(&self, user_id: Uuid) -> Result<Vec<Device>, DeviceRepositoryError> {
        let query =
            "SELECT id, user_id, physical_id, name, event_format, event_data FROM devices WHERE user_id = $1";
        let rows = sqlx::query(query)
            .bind(sqlx::types::Uuid::from(user_id))
            .fetch_all(&self.pool)
            .await
            .map_err(|e| {
                println!("Error fetching devices by user ID {}: {:?}", user_id, e);
                DeviceRepositoryError::InternalError(e.to_string())
            })?;

        let mut devices = Vec::new();
        for row in rows {
            let id: Uuid = row.get("id");
            let user_id: Uuid = row.get("user_id");
            let physical_id: String = row.get("physical_id");
            let name: String = row.get("name");
            let event_format: String = row.get("event_format");
            let event_data_raw: sqlx::types::Json<HashMap<String, String>> = row.get("event_data");

            let mut event_data = HashMap::new();
            for (key, value) in event_data_raw.0 {
                let event_data_type = EventDataType::from_str(&value)
                    .map_err(|e| DeviceRepositoryError::InternalError(e.to_string()))?;
                event_data.insert(key, event_data_type);
            }

            devices.push(Device::new(
                &id,
                &physical_id,
                &user_id,
                &name,
                EventFormat::try_from(event_format.as_str())
                    .map_err(|e| DeviceRepositoryError::InternalError(e.to_string()))?,
                event_data,
            ))
        }
        Ok(devices)
    }
}

impl DeleteDeviceRepository for PostgresDeviceRepository {
    async fn delete_by_id(&self, id: Uuid) -> Result<(), DeviceRepositoryError> {
        let query = "DELETE FROM devices WHERE id = $1";
        let result: PgQueryResult = sqlx::query(query)
            .bind(sqlx::types::Uuid::from(id))
            .execute(&self.pool)
            .await
            .map_err(|e| DeviceRepositoryError::InternalError(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(DeviceRepositoryError::NotFound);
        }

        Ok(())
    }
}

impl UpdateDeviceRepository for PostgresDeviceRepository {
    fn update(
        &self,
        device: &Device,
    ) -> impl Future<Output = Result<(), DeviceRepositoryError>> + Send {
        self.create(device)
    }
}
