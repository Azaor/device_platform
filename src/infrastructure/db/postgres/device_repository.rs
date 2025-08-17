use std::collections::HashMap;

use sqlx::{PgPool, Row, postgres::PgQueryResult};
use uuid::Uuid;

use crate::{
    application::ports::outbound::device_repository::{
        CreateDeviceRepository, DeleteDeviceRepository, DeviceRepositoryError, GetDeviceRepository,
        UpdateDeviceRepository,
    },
    domain::{
        action::action_emittable::ActionEmittable, device::Device,
        event::event_emittable::EventEmittable,
    },
    infrastructure::db::postgres::utils::{
        serialize_action_data, serialize_event_data, ActionEmittableDb, EventEmittableDb
    },
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
                events JSONB NOT NULL DEFAULT '{}',
                actions JSONB NOT NULL DEFAULT '{}',
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
        let query = "INSERT INTO devices (id, user_id, physical_id, name, events, actions) VALUES ($1, $2, $3, $4, $5, $6) ON CONFLICT (id) DO UPDATE SET physical_id = $3, name = $4, events = $5, actions = $6";
        let result: PgQueryResult = sqlx::query(query)
            .bind(sqlx::types::Uuid::from(*device.id()))
            .bind(sqlx::types::Uuid::from(*device.user_id()))
            .bind(&device.physical_id())
            .bind(&device.name())
            .bind(sqlx::types::Json::from(serialize_event_data(
                &device.events(),
            )))
            .bind(sqlx::types::Json::from(serialize_action_data(
                &device.actions(),
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
        let query = "SELECT id, user_id, physical_id, name, events, actions FROM devices WHERE id = $1";
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
        let event_data_raw: sqlx::types::Json<HashMap<String, EventEmittableDb>> =
            row.get("events");

        let mut events = HashMap::new();
        for (key, value) in event_data_raw.0 {
            let event_data_type = EventEmittable::try_from(value)
                .map_err(|e| DeviceRepositoryError::InternalError(e.to_string()))?;
            events.insert(key, event_data_type);
        }

        let actions_raw: sqlx::types::Json<HashMap<String, ActionEmittableDb>> = row.get("actions");
        let mut actions = HashMap::new();
        for (key, value) in actions_raw.0 {
            let action = ActionEmittable::try_from(value)
                .map_err(|e| DeviceRepositoryError::InternalError(e.to_string()))?;
            actions.insert(key, action);
        }
        // Creating the Device instance
        let device = Device::new(&id, &physical_id, &user_id, &name, events, actions);
        Ok(Some(device))
    }

    async fn get_by_user_id(&self, user_id: Uuid) -> Result<Vec<Device>, DeviceRepositoryError> {
        let query = "SELECT id, user_id, physical_id, name, events, actions FROM devices WHERE user_id = $1";
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
            let events_raw: sqlx::types::Json<HashMap<String, EventEmittableDb>> =
                row.get("events");
            let mut events = HashMap::new();
            for (key, value) in events_raw.0 {
                let event = EventEmittable::try_from(value)
                    .map_err(|e| DeviceRepositoryError::InternalError(e.to_string()))?;
                events.insert(key, event);
            }

            let actions_raw: sqlx::types::Json<HashMap<String, ActionEmittableDb>> =
                row.get("actions");
            let mut actions = HashMap::new();
            for (key, value) in actions_raw.0 {
                let action = ActionEmittable::try_from(value)
                    .map_err(|e| DeviceRepositoryError::InternalError(e.to_string()))?;
                actions.insert(key, action);
            }

            devices.push(Device::new(
                &id,
                &physical_id,
                &user_id,
                &name,
                events,
                actions,
            ))
        }
        Ok(devices)
    }

    async fn get_by_physical_id(
        &self,
        physical_id: &str,
    ) -> Result<Option<Device>, DeviceRepositoryError> {
        let query =
            "SELECT id, user_id, physical_id, name, events, actions FROM devices WHERE physical_id = $1";
        let row = sqlx::query(query)
            .bind(physical_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| {
                println!(
                    "Error fetching devices by physical ID {}: {:?}",
                    physical_id, e
                );
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
        let event_data_raw: sqlx::types::Json<HashMap<String, EventEmittableDb>> =
            row.get("events");

        let mut events = HashMap::new();
        for (key, value) in event_data_raw.0 {
            let event = EventEmittable::try_from(value)
                .map_err(|e| DeviceRepositoryError::InternalError(e.to_string()))?;
            events.insert(key, event);
        }
        let actions_raw: sqlx::types::Json<HashMap<String, ActionEmittableDb>> = row.get("actions");
        let mut actions = HashMap::new();
        for (key, value) in actions_raw.0 {
            let action = ActionEmittable::try_from(value)
                .map_err(|e| DeviceRepositoryError::InternalError(e.to_string()))?;
            actions.insert(key, action);
        }
        // Creating the Device instance
        let device = Device::new(&id, &physical_id, &user_id, &name, events, actions);
        Ok(Some(device))
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
