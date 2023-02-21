use rocket::get;

#[get("/")]
pub fn get() -> String {
    String::from("Hello, world!")
}
