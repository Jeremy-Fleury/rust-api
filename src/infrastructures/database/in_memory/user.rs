use crate::domain::{
    entities::user_entity::User,
    repositories::user_repository::{UserInput, UserRepository},
};

pub struct UserInMemoryRepository {
    users: Vec<User>,
}

impl UserRepository for UserInMemoryRepository {
    fn create_user(&mut self, user_input: UserInput) -> User {
        let user = User {
            id: "123".to_string(),
            email: user_input.email,
            password: user_input.password,
        };

        self.users.push(user.clone());

        return user;
    }

    fn get_all_user(&self) -> Vec<User> {
        self.users.to_vec()
    }
}
