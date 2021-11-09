use rocket::serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Queryable)]
#[serde(crate = "rocket::serde")]
pub struct Post {
    #[serde(skip_deserializing)]
    id: Option<i32>,
    title: String,
    text: String,
    #[serde(skip_deserializing)]
    published: bool,
}
