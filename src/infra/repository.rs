use crate::domain::model::User;
use crate::domain::repositories::UserRepository;
use crate::utils::logger::Logger;

pub struct UserRepositoryImpl {
    users: Vec<User>,
    logger: Logger
}

impl UserRepositoryImpl {
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

impl UserRepository for UserRepositoryImpl {
    fn get_all_users(&self) -> Vec<User> {
        self.logger.log("Getting all users");
        self.users.clone()
    }

    fn add_user(&mut self, user: User) {
        self.logger.log(format!("Adding user: {:?}", user));
        self.users.push(user)
    }
}