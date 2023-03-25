#![allow(dead_code, unused_variables)]

// use rocket::{Request, request::FromRequest};

#[derive(Debug, PartialEq, FromForm)]
pub struct User<'r> {
    name: &'r str
}

#[get("/user/<name>")]
pub async fn get(name: String) -> String {
    format!("{}", name)
}

#[patch("/user/<name>/update?<user>")]
pub async fn update(name: String, user: User<'_>) -> String {
    format!("{}, {:?}", name, user)
}

#[delete("/user/<name>/delete")]
pub async fn delete(name: String) -> String {
    format!("{}", name)
}

#[post("/user/create?<user>")]
pub async fn create(user: User<'_>) -> String {
    format!("{:#?}", user)
}
