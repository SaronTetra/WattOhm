#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

use crate::post::model::Post;
use diesel::prelude::*;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket_sync_db_pools::database;

pub mod post;
mod schema;

#[database("postgres")]
struct DbConnection(diesel::PgConnection);

#[get("/")]
fn world() -> &'static str {
    "Hello, world!"
}

#[get("/<id>")]
fn db(id: i32) -> String {
    format!("id:{0}", id)
}

#[get("/post/<post_id>")]
async fn get_post_db(conn: DbConnection, post_id: i32) -> Result<Post, diesel::result::Error> {
    // conn.run(|c| posts::filter(published.eq(true))).await
    // conn.run(|c| posts::filter(posts::id.eq(post_id))).await
    use crate::schema::posts::dsl::*;
    conn.run(|c| {
        schema::posts::table
            .filter(published.eq(true))
            .first(c)
            .unwrap()
    })
    .await
    // conn.run(|c| posts.filter(post_id.eq(&id)).load(c)).await
    // diesel::select(posts.filter(id.eq(post_id)))
}

#[allow(unused_must_use)]
#[rocket::main]
async fn main() {
    rocket::build()
        .mount("/", routes![world])
        .mount("/db", routes![db])
        .attach(DbConnection::fairing())
        .launch()
        .await;
}
