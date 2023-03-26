#![allow(dead_code, unused_variables)]

use rocket::http::Status;
use rocket::serde::{json::Json, Serialize};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Response<T> {
    status_code: String,
    data: T,
}

#[derive(Debug, PartialEq, FromForm, Serialize)]
pub struct User<'r> {
    name: &'r str,
}

#[get("/user/<name>")]
pub async fn get(name: String) -> String {
    format!("{}", name)
}

#[put("/user/<name>/update?<user>")]
pub fn update(name: String, user: User<'_>) -> Json<Response<User>> {
    let response = Response {
        status_code: Status::Ok.to_string(),
        data: user,
    };
    Json(response)
}

#[delete("/user/<name>/delete")]
pub fn delete(name: String) -> Json<Response<String>> {
    let response = Response {
        status_code: Status::Ok.to_string(),
        data: name,
    };
    Json(response)
}

#[post("/user/create?<user>")]
pub fn create(user: User<'_>) -> Json<Response<User>> {
    let response = Response {
        status_code: Status::Ok.to_string(),
        data: user,
    };
    Json(response)
}
