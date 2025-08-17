use uuid::Uuid;

use crate::domain::action::{
    action::Action,
    action_format::{ActionFormat, ActionFormatError},
};

pub enum ActionRepositoryError {
    RepositoryError(String),
    ValidationError(String),
}

pub trait HandleActionRepository: Send + Sync {
    fn get_actions(
        &mut self,
        device_id: &str,
    ) -> impl Future<Output = Result<Vec<Action>, ActionRepositoryError>> + Send;

}

pub trait CreateActionRepository: Send + Sync {
    fn create_action(
        &self,
        action: Action,
        action_format: &ActionFormat,
    ) -> impl Future<Output = Result<(), ActionRepositoryError>> + Send;
}

impl From<ActionFormatError> for ActionRepositoryError {
    fn from(value: ActionFormatError) -> Self {
        match value {
            ActionFormatError::UnsupportedFormat(e) => {
                ActionRepositoryError::RepositoryError(format!("Unsupported format: {}", e))
            }
        }
    }
}
