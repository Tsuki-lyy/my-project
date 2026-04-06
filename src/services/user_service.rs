use std::collections::HashMap;
use std::rc::Rc;

use crate::models::User;

pub struct UserService {
    users: HashMap<String, Rc<dyn User>>,
}

impl UserService {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
        }
    }

    pub fn add_user<T: User + 'static>(&mut self, user: T) {
        let user_rc = Rc::new(user);
        self.users.insert(user_rc.id().to_string(), user_rc);
    }

    pub fn get_user(&self, user_id: &str) -> Option<Rc<dyn User>> {
        self.users.get(user_id).cloned()
    }

    pub fn get_user_count(&self) -> usize {
        self.users.len()
    }

    pub fn get_all_users(&self) -> Vec<Rc<dyn User>> {
        self.users.values().cloned().collect()
    }
}