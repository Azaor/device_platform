use uuid::Uuid;
use std::{collections::HashMap};

use crate::domain::{action::action_emittable::ActionEmittable, event::event_emittable::EventEmittable};

#[derive(Debug, Clone)]
pub struct Device {
    id: Uuid,
    physical_id: String,
    user_id: Uuid,
    name: String,
    events: HashMap<String, EventEmittable>,
    actions: HashMap<String, ActionEmittable>,
}

impl Device {
    pub fn new(id: &Uuid, physical_id: &str, user_id: &Uuid, name: &str, events: HashMap<String, EventEmittable>, actions: HashMap<String, ActionEmittable>) -> Self {
        return Self { id: id.clone(), physical_id: physical_id.to_string(), user_id: user_id.clone(), name: name.to_string(), events, actions }
    }
    pub fn id(&self) -> &Uuid {
        &self.id
    }
    pub fn physical_id(&self) -> &str {
        &self.physical_id
    }
    pub fn set_physical_id(&mut self, physical_id: &str) {
        self.physical_id = physical_id.to_string();
    }
    pub fn user_id(&self) -> &Uuid {
        &self.user_id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
    pub fn events(&self) -> &HashMap<String, EventEmittable> {
        &self.events
    }
    pub fn set_events(&mut self, events: HashMap<String, EventEmittable>) {
        self.events = events;
    }
    pub fn actions(&self) -> &HashMap<String, ActionEmittable> {
        &self.actions
    }
    pub fn set_actions(&mut self, actions: HashMap<String, ActionEmittable>) {
        self.actions = actions;
    }

}
