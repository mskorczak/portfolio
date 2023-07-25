#[macro_use] extern crate rocket;
extern crate rocket_contrib;

use std::path::{Path, PathBuf};
use rocket::fs::FileServer;
use rocket_dyn_templates::Template;
use rocket_contrib::serve::StaticFiles;

mod blog;

#[get("/")]
fn index() -> &'static str {
    "Hello, Bozo!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![blog::index, blog::article, blog::api, blog::api_id])
        .mount("/", routes![index])
        .mount("/static", FileServer::from("static/"))

        .register("/", catchers![blog::not_found])
        .attach(Template::fairing())
}

