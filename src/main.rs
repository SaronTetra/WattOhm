#[macro_use]
extern crate rocket;
mod card;

// extern crate diesel

#[get("/")]
fn world() -> &'static str {
    "Hello, world!"
}

#[get("/<id>")]
fn db(id: i32) -> String {
    format!("id:{0}", id)
}

#[get("/")]
fn cardgen() -> String {
    let number:String=card::numbergen::generate();
    format!("Card number: {0}", number)
}

#[allow(unused_must_use)]
#[rocket::main]
async fn main() {
    rocket::build()
        .mount("/", routes![world])
        .mount("/db", routes![db])
        .mount("/card", routes![cardgen])
        .launch()
        .await;
}
