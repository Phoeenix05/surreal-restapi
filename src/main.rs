// use anyhow::Result;

#[macro_use]
extern crate rocket;

#[get("/api/hello")]
async fn hello() -> String {
    format!("hello")
}

#[launch]
async fn rocket() -> _ {
    rocket::build().mount("/", routes![hello])
}
