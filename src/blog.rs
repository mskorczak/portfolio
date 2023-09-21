use rocket::Request;
use rocket::response::Redirect;
use std::path::{Path, PathBuf};
use rocket::fs::NamedFile;
use rocket_dyn_templates::{Template, tera::Tera, context};
use std::io;
use std::fs;
use std::fs::File;
use rocket::http::RawStr;
use rocket::response::status::NotFound;


#[catch(404)]
pub fn not_found(req: &Request<'_>) -> Template {
    Template::render("error/404", context! {
        uri: req.uri()
    })
}

#[get("/api")]
pub fn api() -> &'static str {
    "FUCK YOU!!!!!!!!!!!!!!!!!!!!"
}

#[get("/api/<id>")]
pub fn api_id(id: Result<u8, &str>) -> String {
    match id {
        Ok(id_num) => format!("u8: {}", id_num),
        Err(string) => format!("not a u8: {}", string)
    }
}
#[get("/blog")]
pub fn blog() -> Template {
    let mut filenames: Vec<String> = vec![];
    for file in fs::read_dir("blog/").unwrap() {
        match file {
            Ok(filename) => {
                filenames.push(format!("{}", filename.file_name().into_string().unwrap()));
            },
            Err(error) => filenames.push(format!("Error loading blog post."))
        }
    }
    Template::render("blog", context! {
        file_names: filenames
    })
}

#[get("/article/<article>")]
pub async fn article(article:PathBuf) -> Result<NamedFile, NotFound<String>> {
    let file_name = article.as_path().display().to_string();
    let path: PathBuf = [r"blog/",&file_name].iter().collect();
    NamedFile::open(&path).await.map_err(|e| NotFound(e.to_string()))
}

#[get("/home")]
pub fn home() -> Template {
    Template::render("home",context! {})
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

