use super::model::*;
use super::schema::*;
use crate::diesel::prelude::*;
use crate::DbConnection;
use rocket::serde::json::Json;

#[get("/")]
pub async fn list(db: DbConnection) -> Option<Json<Vec<Post>>> {
    db.run(|c| posts::table.load(c)).await.map(Json).ok()
}

#[get("/<id>")]
pub async fn read(db: DbConnection, id: i32) -> Option<Json<Post>> {
    db.run(move |conn| posts::table.filter(posts::id.eq(id)).first(conn))
        .await
        .map(Json)
        .ok()
}

#[get("/")]
pub fn world() -> &'static str {
    "Hello, world!"
}
