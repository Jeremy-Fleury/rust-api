use crate::domain::entities::user_entity::User;

#[derive(Debug)]
pub struct UserInput {
    pub email: String,
    pub password: String,
}

pub trait UserRepository {
    fn create_user(&mut self, user_input: UserInput) -> User;

    fn get_all_user(&self) -> Vec<User>;
}
