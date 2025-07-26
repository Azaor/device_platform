use std::collections::HashMap;

use serde_json::Value;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::{application::ports::outbound::event_repository::{CreateEventRepository, EventRepositoryError, GetEventRepository}, domain::{device::EventFormat, event::{Event, EventDataValue}}};

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
                device_id UUID NOT NULL,
                timestamp TIMESTAMPTZ NOT NULL,
                payload JSONB NOT NULL,
                UNIQUE (id, device_id)
            )",
        )
        .execute(&self.pool)
        .await
        .expect("Failed to create events table");
    }
}

impl CreateEventRepository for PostgresEventRepository {
    async fn create_event(&self, evt: Event, event_format: &EventFormat) -> Result<(), EventRepositoryError> {
        let query = "INSERT INTO events (id, device_id, timestamp, payload) VALUES ($1, $2, $3, $4)
                     ON CONFLICT (id, device_id) DO NOTHING";
        
        sqlx::query(query)
            .bind(evt.id)
            .bind(evt.device_id)
            .bind(evt.timestamp)
            .bind(event_format.encode_event(evt.payload)?)
            .execute(&self.pool)
            .await
            .map_err(|e| EventRepositoryError::RepositoryError(e.to_string()))?;
        
        Ok(())
    }
}

impl GetEventRepository for PostgresEventRepository {
    async fn get_events(&self, uid: &Uuid) -> Result<Vec<Event>, EventRepositoryError> {
        let query = "SELECT id, device_id, timestamp, payload FROM events WHERE device_id = $1";
        let rows = sqlx::query(query)
            .bind(uid)
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
                device_id: row.get("device_id"),
                timestamp: row.get("timestamp"),
                payload,
            })
        }).collect();
        
        events
    }
}