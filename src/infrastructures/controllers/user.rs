use rocket::serde::json::Json;

use crate::domain::entities::user_entity::User;

#[get("/")]
pub fn get_all_users() -> Json<Vec<User>> {
    let users = vec![User {
        id: "1".to_string(),
        email: "".to_string(),
        password: "".to_string(),
    }];

    Json(users)
}
