#[macro_use] extern crate rocket;

use std::path::{Path, PathBuf};
use rocket::fs::FileServer;
use rocket_dyn_templates::Template;

mod blog;

#[get("/")]
fn index() -> &'static str {
    "Hello, Bozo!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![blog::index, blog::article])
        .register("/", catchers![blog::not_found])
        .attach(Template::fairing())
        .mount("/", routes![index])
}

