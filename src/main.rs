#[macro_use]
extern crate rocket;

mod infrastructures;

use infrastructures::controllers;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![controllers::hello_world_controller::get])
}
