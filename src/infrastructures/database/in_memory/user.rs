use crate::domain::{
    entities::user_entity::User,
    repositories::user_repository::{PartialUserInput, UserInput, UserRepository},
};
use std::collections::HashMap;

pub struct UserInMemoryRepository {
    users: HashMap<String, User>,
}

impl UserRepository for UserInMemoryRepository {
    fn get_by_id(&self, id: &str) -> User {
        let user = self.users.get(id);

        return match user {
            Some(u) => u.clone(),
            None => panic!("No user found"),
        };
    }

    fn get_all(&self) -> Vec<User> {
        let users = Vec::from_iter(self.users.clone().into_values());

        return users;
    }

    fn create(&mut self, user_input: UserInput) -> User {
        let id = "123";
        let user = User {
            id: id.to_string(),
            email: user_input.email,
            password: user_input.password,
        };

        self.users.insert(id.to_string(), user.clone());
        return user;
    }

    fn update(&mut self, id: &str, partial_user_input: PartialUserInput) -> User {
        let user = self.users.get_mut(id);

        match user {
            Some(u) => {
                *u = User {
                    id: u.id.clone(),
                    email: partial_user_input.email.unwrap_or(u.email.clone()),
                    password: partial_user_input.password.unwrap_or(u.password.clone()),
                };

                u.clone()
            }
            None => panic!("No user found"),
        }
    }
}
