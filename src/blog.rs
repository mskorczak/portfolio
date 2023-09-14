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
/*
#[get("/blog/<article>")]
pub fn article(article: PathBuf) -> Template {
    let file_name = article.as_path().display().to_string();
    let path: PathBuf = [r"blog/",&file_name].iter().collect();
    println!("{}",path.display());
    let content: String = fs::read_to_string(path.as_path()).unwrap();
    let parser = pulldown_cmark::Parser::new(&content);
    let mut data = String::new();
    pulldown_cmark::html::push_html(&mut data, parser);
    println!("{}",data);
    Template::render("blog/article", context! {
        content: data
    })
}
#[get("/blog/<article>")]
pub async fn article(article: PathBuf) -> Result<NamedFile, NotFound<String>> {
    let path = Path::new("blog/").join(article);
    NamedFile::open(&path).await.map_err(|e| NotFound(e.to_string()))
}
*/

//#[get("/blog/<article>")]
pub async fn article(article:PathBuf) -> String {
    let file_name = article.as_path().display().to_string();
    let path: PathBuf = [r"blog/", &file_name].iter().collect();
    let content = fs::read_to_string(path.as_path());
    match content {
        Ok(content_string) => format!("{}", content_string),
        Err(error) => format!("ERROR: {}", error)
    }
    /*match content {
        Ok(content_string) => Template::render("blogpost", context! {content:content_string}),
        Err(error) => Template::render("error", context! {
            content : format!("ERROR:{}",error)
        })
    }*/
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
//#[get("/blog")]
pub fn index() -> Template {

    let file_names: Vec<_> = fs::read_dir("blog/")
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .collect();
    
    Template::render("index", context! {
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

