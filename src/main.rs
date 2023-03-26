#[macro_use]
extern crate rocket;

mod api;
mod database;

use database::DbFairing;
use rocket::{
    figment::{
        providers::{Format, Toml},
        Figment,
    },
    Config,
};

#[get("/")]
fn index() -> String {
    format!("")
}

#[launch]
async fn rocket() -> _ {
    let figment = Figment::from(Config::default())
        .merge(Toml::file("App.toml").nested())
        .merge(Toml::file("Rocket.toml").nested());

    rocket::custom(figment)
        .mount("/", routes![index])
        .mount("/api", api::routes())
        .attach(DbFairing)
}
