use std::fmt::Display;

use uuid::Uuid;

use crate::domain::{
    action::{action::Action, action_format::ActionFormat},
};

#[derive(Debug)]
pub enum ActionServiceError {
    InvalidInput(String),
    InternalError(String),
}

impl Display for ActionServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActionServiceError::InvalidInput(v) => f.write_str(&format!("Invalid input: {}", &v)),
            ActionServiceError::InternalError(v) => f.write_str(&format!("Internal error: {}", &v)),
        }
    }
}

pub trait ActionService {
    async fn send_action(
        &self,
        event: Action,
        event_format: &ActionFormat,
    ) -> Result<(), ActionServiceError>;
    async fn get_actions(&self, device_id: &str) -> Result<Vec<Action>, ActionServiceError>;
}
