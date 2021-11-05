#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket_sync_db_pools;

use rocket::response::Debug;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket_sync_db_pools::database;

use self::diesel::prelude::*;

#[database("postgres")]
struct DbConnection(diesel::PgConnection);

type Result<T, E = Debug<diesel::result::Error>> = std::result::Result<T, E>;

#[derive(Serialize, Deserialize, Queryable)]
#[serde(crate = "rocket::serde")]
struct Post {
    #[serde(skip_deserializing)]
    id: Option<i32>,
    title: String,
    text: String,
    #[serde(skip_deserializing)]
    published: bool,
}

table! {
    posts (id) {
        id -> Nullable<Int4>,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

// #[get("/db")]
// async fn list(db: DbConnection) -> Result<Json<Vec<Option<i32>>>> {
//     let ids: Vec<Option<i32>> = db
//         .run(move |c| posts::table.select(posts::id).load(c))
//         .await?;
//
//     Ok(Json(ids))
// }

#[get("/<id>")]
async fn read(db: DbConnection, id: i32) -> Option<Json<Post>> {
    db.run(move |conn| posts::table.filter(posts::id.eq(id)).first(conn))
        .await
        .map(Json)
        .ok()
}

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
        .mount("/", routes![world, read])
        .mount("/db", routes![db])
        .attach(DbConnection::fairing())
        .launch()
        .await;
}
