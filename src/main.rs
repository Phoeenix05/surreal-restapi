// use anyhow::Result;

#[macro_use]
extern crate rocket;

mod api;

#[get("/")]
fn index() -> String {
    format!("")
}

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/api", api::routes())
}
