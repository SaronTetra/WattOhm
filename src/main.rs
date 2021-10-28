#[macro_use]
extern crate rocket;

#[get("/")]
fn world() -> &'static str {
    "Hello, world!"
}

#[rocket::main]
async fn main() {
    rocket::build().mount("/", routes![world]).launch().await;
}
