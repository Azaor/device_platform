use std::{collections::HashMap, sync::Mutex};

use crate::{application::ports::outbound::action_repository::{ActionRepositoryError, CreateActionRepository, HandleActionRepository}, domain::action::{action::Action, action_format::ActionFormat}};



#[derive(Debug)]
pub struct InMemoryActionRepository {
    actions: Mutex<HashMap<String, Vec<Action>>>,
}

impl InMemoryActionRepository {
    pub fn new() -> Self {
        return InMemoryActionRepository {
            actions: Mutex::new(HashMap::new()),
        };
    }
}
impl CreateActionRepository for InMemoryActionRepository {
    async fn create_action(
        &self,
        action: Action,
        _: &ActionFormat,
    ) -> Result<(), ActionRepositoryError> {
        let mut actions = self.actions.lock().unwrap();
        match actions.get_mut(&action.device_id) {
            Some(device_actions) => device_actions.push(action),
            None => {
                actions.insert(action.device_id.clone(), vec![action]);
            }
        }
        return Ok(());
    }
}

impl HandleActionRepository for InMemoryActionRepository {
    async fn get_actions(
        &mut self,
        device_id: &str,
    ) -> Result<Vec<Action>, ActionRepositoryError> {
        let actions = self.actions.lock().unwrap();
        let actions_found = match actions.get(device_id) {
            Some(device_actions) => device_actions,
            None => &Vec::new(),
        };
        return Ok(actions_found.clone());
    }
}
