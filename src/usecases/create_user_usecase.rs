pub struct CreateUser {
    repo: Box<dyn UserRepository>,
}

impl CreateUser {
    pub fn new(repo: Box<dyn UserRepository>) -> Self {
        Self { repo }
    }

    pub fn execute(&self, email: &str, password: &str) {
        let user = User::new(email, password);
        self.repo.save(user);
    }
}
