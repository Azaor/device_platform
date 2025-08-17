use std::{collections::HashSet, sync::Arc};
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::{
    application::ports::{
        inbound::action_service::{ActionService, ActionServiceError},
        outbound::action_repository::{
            ActionRepositoryError, CreateActionRepository, HandleActionRepository,
        },
    },
    domain::action::{action::Action, action_format::ActionFormat},
};

#[derive(Debug)]
pub struct ManageActionService<C: CreateActionRepository, H: HandleActionRepository> {
    pub create_repo: Arc<Mutex<C>>,
    pub get_repo: Arc<Mutex<H>>,
}

impl<C: CreateActionRepository, H: HandleActionRepository> ActionService
    for ManageActionService<C, H>
{
    async fn send_action(
        &self,
        action: Action,
        event_format: &ActionFormat,
    ) -> Result<(), ActionServiceError> {
        let repo = self.create_repo.lock().await;
        if let Err(e) = repo
            .create_action(action.clone(), event_format)
            .await
        {
            return Err(match e {
                ActionRepositoryError::RepositoryError(msg) => {
                    ActionServiceError::InternalError(msg)
                }
                ActionRepositoryError::ValidationError(msg) => {
                    ActionServiceError::InvalidInput(msg)
                }
            });
        }
        Ok(())
    }

    async fn get_actions(
        &self,
        device_id: &str,
    ) -> Result<Vec<Action>, ActionServiceError> {
        let mut result = Vec::new();
        let mut repo = self.get_repo.lock().await;
        match repo.get_actions(device_id).await {
            Ok(actions) => result.push(actions),
            Err(ActionRepositoryError::RepositoryError(msg)) => {
                return Err(ActionServiceError::InternalError(msg));
            }
            Err(ActionRepositoryError::ValidationError(msg)) => {
                return Err(ActionServiceError::InvalidInput(msg));
            }
        }
        let mut actions = HashSet::new();
        for repo_actions in result {
            for action in repo_actions {
                actions.insert(action);
            }
        }
        Ok(actions.into_iter().collect())
    }
}
