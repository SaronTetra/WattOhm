#[macro_use]
extern crate rocket;

// extern crate diesel

#[get("/")]
fn world() -> &'static str {
    "Hello, world!"
}

#[get("/<id>")]
fn db(id: i32) -> String {
    format!("id:{0}", id)
}

#[allow(unused_must_use)]
#[rocket::main]
async fn main() {
    rocket::build()
        .mount("/", routes![world])
        .mount("/db", routes![db])
        .launch()
        .await;
}
