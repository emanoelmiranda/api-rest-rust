use crate::domain::model::User;
use crate::utils::logger::Logger;

pub struct UserRepository {
    users: Vec<User>,
    logger: Logger
}

impl UserRepository {
    pub fn new(logger: Logger) -> Self {
        Self {
            users: Vec::new(),
            logger
        }
    }

    pub fn get_all_users(&self) -> Vec<User> {
        self.logger.log("Getting all users");
        self.users.clone()
    }

    pub fn add_user(&mut self, user: User) {
        self.logger.log(format!("Adding user: {:?}", user));
        self.users.push(user)
    }
}