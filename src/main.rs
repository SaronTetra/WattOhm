#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

use rocket_sync_db_pools::database;

mod posts;
use crate::posts::handlers::*;

#[database("postgres")]
pub struct DbConnection(diesel::PgConnection);

#[allow(unused_must_use)]
#[rocket::main]
async fn main() {
    rocket::build()
        .mount("/", routes![world,])
        .mount("/db", routes![list, read])
        .attach(DbConnection::fairing())
        .launch()
        .await;
}
