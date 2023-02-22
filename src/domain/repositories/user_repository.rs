use crate::domain::entities::user_entity::User;

#[derive(Debug)]
pub struct UserInput {
    pub email: String,
    pub password: String,
}

pub struct PartialUserInput {
    pub email: Option<String>,
    pub password: Option<String>,
}

pub trait UserRepository {
    fn get_by_id(&self, id: &str) -> User;

    fn get_all(&self) -> Vec<User>;

    fn create(&mut self, user_input: UserInput) -> User;

    fn update(&mut self, id: &str, partial_user_input: PartialUserInput) -> User;
}
