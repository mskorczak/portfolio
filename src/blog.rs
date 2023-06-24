use rocket::Request;
use rocket::response::Redirect;
use std::path::{Path, PathBuf};
use rocket::fs::NamedFile;
use rocket_dyn_templates::{Template, tera::Tera, context};
use std::io;
use std::fs;

#[catch(404)]
pub fn not_found(req: &Request<'_>) -> Template {
    Template::render("blog/error/404", context! {
        uri: req.uri()
    })
}

#[get("/blog/<article>")]
pub async fn article(article: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("blog/").join(article)).await.ok()
}

#[get("/blog")]
pub fn index() -> Template {

    let file_names: Vec<_> = fs::read_dir("blog/")
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .collect();
    
    Template::render("blog/index", context! {
        items: file_names
    })
}

pub fn customize(tera: &mut Tera) {
    tera.add_raw_template("tera/about.html", r#"
        {% extends "tera/base" %}
        {% block content %}
        <section id="about">
        <h1>About - Here's another page!</h1>
        </section>
        {% endblock content %}
    "#).expect("valid Tera template");
}

