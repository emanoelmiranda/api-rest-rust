use crate::domain::model::User;
use crate::infra::repository::UserRepositoryImpl;

pub struct UserUseCase {
    repository: UserRepositoryImpl
}

impl UserUseCase {
    pub fn new(repository: UserRepositoryImpl) -> Self {
        Self { repository }
    }

    pub fn get_users(&self) -> Vec<User> {
        self.repository.get_all_users()
    }

    pub fn add_users(&mut self, user: User)  {
        self.repository.add_user(user)
    }

    pub fn add_user(&self) {
        // self.repository.add_user(user);
    }
}
