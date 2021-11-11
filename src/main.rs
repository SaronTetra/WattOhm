#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

use rocket_sync_db_pools::database;
mod card;

mod posts;
use crate::posts::handlers::*;

#[database("postgres")]
pub struct DbConnection(diesel::PgConnection);

#[get("/")]
fn cardgen() -> String {
    let number: String = card::numbergen::generate();
    format!("Card number: {0}", number)
}

#[allow(unused_must_use)]
#[rocket::main]
async fn main() {
    rocket::build()
        .mount("/", routes![world,])
        .mount("/db", routes![list, read])
        .mount("/card", routes![cardgen])
        .attach(DbConnection::fairing())
        .launch()
        .await;
}
