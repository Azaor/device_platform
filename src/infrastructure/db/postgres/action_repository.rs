use std::collections::HashMap;

use serde_json::Value;
use sqlx::{PgPool, Row};

use crate::{
    application::ports::outbound::action_repository::{
        ActionRepositoryError, CreateActionRepository, HandleActionRepository,
    },
    domain::action::{
        action::Action, action_data_value::ActionDataValue, action_format::ActionFormat,
    },
};

#[derive(Debug)]
pub struct PostgresActionRepository {
    pool: sqlx::PgPool,
}
impl PostgresActionRepository {
    pub async fn new(pool: PgPool) -> Self {
        PostgresActionRepository { pool }
    }
    pub async fn init(&self) {
        // Ensure the events table exists
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS actions (
                id UUID PRIMARY KEY,
                device_id TEXT NOT NULL,
                action_name TEXT NOT NULL,
                timestamp TIMESTAMPTZ NOT NULL,
                payload JSONB NOT NULL,
                UNIQUE (id, device_id)
            )",
        )
        .execute(&self.pool)
        .await
        .expect("Failed to create actions table");
    }
}

impl CreateActionRepository for PostgresActionRepository {
    async fn create_action(
        &self,
        action: Action,
        _: &ActionFormat,
    ) -> Result<(), ActionRepositoryError> {
        let query = "INSERT INTO actions (id, device_id, action_name, timestamp, payload) VALUES ($1, $2, $3, $4, $5)
                     ON CONFLICT (id, device_id) DO NOTHING";
        let event_data: HashMap<String, Value> = action
            .payload
            .into_iter()
            .map(|(k, v)| (k, v.into()))
            .collect();
        sqlx::query(query)
            .bind(action.id)
            .bind(action.device_id)
            .bind(action.action_name)
            .bind(action.timestamp)
            .bind(sqlx::types::Json::from(event_data))
            .execute(&self.pool)
            .await
            .map_err(|e| ActionRepositoryError::RepositoryError(e.to_string()))?;

        Ok(())
    }
}

impl HandleActionRepository for PostgresActionRepository {
    async fn get_actions(&mut self, device_id: &str) -> Result<Vec<Action>, ActionRepositoryError> {
        let query = "SELECT id, device_id, action_name, timestamp, payload FROM events WHERE device_id = $1";
        let rows = sqlx::query(query)
            .bind(device_id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| ActionRepositoryError::RepositoryError(e.to_string()))?;

        let actions: Result<Vec<Action>, ActionRepositoryError> = rows
            .into_iter()
            .map(|row| {
                let payload_db: sqlx::types::Json<HashMap<String, Value>> = row.get("payload");
                let mut payload = HashMap::new();
                for (k, v) in payload_db.0 {
                    let val = ActionDataValue::try_from(v).map_err(|_| {
                        ActionRepositoryError::RepositoryError(format!(
                            "Invalid data stored for key {}",
                            k
                        ))
                    })?;
                    payload.insert(k, val);
                }
                Ok(Action {
                    id: row.get("id"),
                    device_id: row.get("device_id"),
                    action_name: row.get("action_name"),
                    timestamp: row.get("timestamp"),
                    payload,
                })
            })
            .collect();

        actions
    }
}
