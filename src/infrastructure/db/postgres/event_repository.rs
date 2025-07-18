use std::collections::HashMap;

use sqlx::{PgPool, Row};

use crate::{application::ports::outbound::event_repository::{EventRepository, EventRepositoryError}, domain::event::Event};

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
impl EventRepository for PostgresEventRepository {
    async fn create_event(&self, evt: Event) -> Result<(), EventRepositoryError> {
        let query = "INSERT INTO events (id, device_id, timestamp, payload) VALUES ($1, $2, $3, $4)
                     ON CONFLICT (id, device_id) DO NOTHING";
        
        sqlx::query(query)
            .bind(evt.id)
            .bind(evt.device_id)
            .bind(evt.timestamp)
            .bind(sqlx::types::Json(evt.payload))
            .execute(&self.pool)
            .await
            .map_err(|e| EventRepositoryError::RepositoryError(e.to_string()))?;
        
        Ok(())
    }
    
    async fn get_events(&self, uid: &uuid::Uuid) -> Result<Vec<Event>, EventRepositoryError> {
        let query = "SELECT id, device_id, timestamp, payload FROM events WHERE device_id = $1";
        let rows = sqlx::query(query)
            .bind(uid)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| EventRepositoryError::RepositoryError(e.to_string()))?;
        
        let events: Result<Vec<Event>, EventRepositoryError> = rows.into_iter().map(|row| {
            Ok(Event {
                id: row.get("id"),
                device_id: row.get("device_id"),
                timestamp: row.get("timestamp"),
                payload: row.get::<sqlx::types::Json<HashMap<String, String>>, _>("payload").0,
            })
        }).collect();
        
        events
    }
}