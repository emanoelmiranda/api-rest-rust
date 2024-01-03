use crate::domain::model::User;

pub trait UserRepository {
    fn get_all_users(&self) -> Vec<User>;

    fn add_user(&self, user: User);
}