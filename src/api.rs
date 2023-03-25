mod user;
use user::*;

use rocket::Route;

pub fn routes() -> Vec<Route> {
    routes![get, update, delete, create]
}
