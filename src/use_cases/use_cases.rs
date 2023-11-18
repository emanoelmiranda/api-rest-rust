use crate::domain::model::User;
use crate::infra::repository::UserRepository;

pub struct UserUseCase {
    repository: UserRepository
}

impl UserUseCase {
    pub fn new(repository: UserRepository) -> Self {
        Self { repository }
    }

    pub fn get_users(&self) -> Vec<User> {
        self.repository.get_all_users()
    }
}
