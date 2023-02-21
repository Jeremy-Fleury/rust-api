#[macro_use]
extern crate rocket;

mod domain;
mod infrastructures;

use infrastructures::controllers;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/users", routes![controllers::user::get_all_users])
        .launch()
        .await?;

    Ok(())
}
