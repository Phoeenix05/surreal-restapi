#![allow(dead_code, unused_variables)]

use rocket::http::Status;
use rocket::serde::{json::Json, Serialize};
use rocket::State;

use crate::database::Db;

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
pub async fn get(name: String, db: &State<Db>) -> String {
    format!("{}", name)
}

#[put("/user/<name>/update?<user>")]
pub fn update<'a>(name: String, user: User<'a>, db: &State<Db>) -> Json<Response<User<'a>>> {
    let response = Response {
        status_code: Status::Ok.to_string(),
        data: user,
    };
    Json(response)
}

#[delete("/user/<name>/delete")]
pub fn delete(name: String, db: &State<Db>) -> Json<Response<String>> {
    let response = Response {
        status_code: Status::Ok.to_string(),
        data: name,
    };
    Json(response)
}

#[post("/user/create?<user>")]
pub fn create<'a>(user: User<'a>, db: &State<Db>) -> Json<Response<User<'a>>> {
    let response = Response {
        status_code: Status::Ok.to_string(),
        data: user,
    };
    Json(response)
}
