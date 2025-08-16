use std::collections::HashMap;

use serde_json::Value;
use sqlx::{PgPool, Row};

use crate::{application::ports::outbound::event_repository::{CreateEventRepository, EventRepositoryError, GetEventRepository}, domain::{device::EventFormat, event::{Event, EventDataValue}}};

#[derive(Debug)]
pub struct PostgresEventRepository {
    pool: sqlx::PgPool,
}
impl PostgresEventRepository {
    pub async fn new(pool: PgPool) -> Self {
        PostgresEventRepository { pool }
    }
    pub async fn init(&self) {
        // Ensure the events table exists
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS events (
                id UUID PRIMARY KEY,
                device_physical_id TEXT NOT NULL,
                event_name TEXT NOT NULL,
                timestamp TIMESTAMPTZ NOT NULL,
                payload JSONB NOT NULL,
                UNIQUE (id, device_physical_id)
            )",
        )
        .execute(&self.pool)
        .await
        .expect("Failed to create events table");
    }
}

impl CreateEventRepository for PostgresEventRepository {
    async fn create_event(&self, evt: Event, _: &EventFormat) -> Result<(), EventRepositoryError> {
        let query = "INSERT INTO events (id, device_physical_id, event_name, timestamp, payload) VALUES ($1, $2, $3, $4, $5)
                     ON CONFLICT (id, device_physical_id) DO NOTHING";
        let event_data: HashMap<String, Value> = evt.payload.into_iter().map(|(k, v)| (k, v.into())).collect();
        sqlx::query(query)
            .bind(evt.id)
            .bind(evt.device_physical_id)
            .bind(evt.event_name)
            .bind(evt.timestamp)
            .bind(sqlx::types::Json::from(event_data))
            .execute(&self.pool)
            .await
            .map_err(|e| EventRepositoryError::RepositoryError(e.to_string()))?;
        
        Ok(())
    }
}

impl GetEventRepository for PostgresEventRepository {
    async fn get_events(&self, device_physical_id: &str) -> Result<Vec<Event>, EventRepositoryError> {
        let query = "SELECT id, device_physical_id, event_name, timestamp, payload FROM events WHERE device_physical_id = $1";
        let rows = sqlx::query(query)
            .bind(device_physical_id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| EventRepositoryError::RepositoryError(e.to_string()))?;
        
        let events: Result<Vec<Event>, EventRepositoryError> = rows.into_iter().map(|row| {
            let payload_db: sqlx::types::Json<HashMap<String, Value>> = row.get("payload");
            let mut payload = HashMap::new();
            for (k, v) in payload_db.0 {
                let val = EventDataValue::try_from(v).map_err(|_| EventRepositoryError::RepositoryError(format!("Invalid data stored for key {}", k)))?;
                payload.insert(k, val);
            }
            Ok(Event {
                id: row.get("id"),
                device_physical_id: row.get("device_physical_id"),
                event_name: row.get("event_name"),
                timestamp: row.get("timestamp"),
                payload,
            })
        }).collect();
        
        events
    }
}