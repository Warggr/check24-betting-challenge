mod database;
mod schema;
mod orm;
mod betting;

#[macro_use] extern crate rocket;

use rocket::http::CookieJar;
use rocket::http::RawStr;
use rocket::fs::{FileServer, NamedFile};
use rocket::response::Redirect;
use std::path::PathBuf;


#[get("/hello")]
fn hello(cookies: &CookieJar<'_>) -> String {
    match cookies.get("username") {
        None => "You are not logged in.".to_string(),
        Some(username) => format!("You are logged in as {}", username),
    }
}

#[post("/login", data="<username>")]
fn login(username: String, cookies: &CookieJar<'_>) -> Redirect {
    cookies.add(("username", username));
    Redirect::to(uri!(hello()))
}

#[get("/")]
async fn singlepage() -> Option<NamedFile> {
    NamedFile::open("../client/dist/index.html").await.ok()
}

#[get("/favicon.ico")]
async fn favicon() -> Option<NamedFile> {
    NamedFile::open("../client/dist/favicon.ico").await.ok()
}

const FRONTEND_ROOT : &'static str = "/web/";

#[get("/")]
fn redirect_root_to_web() -> Redirect {
    Redirect::to(FRONTEND_ROOT)
}

#[get("/<path..>", rank=11)] // very high rank (higher than the file server) so this will be matched
fn redirect_to_singlepage(path: PathBuf) -> Redirect {
    let mut vue_root = String::from(FRONTEND_ROOT);
    vue_root.push_str("?nav_to="); // TODO: compile-time concat
    vue_root.push_str(RawStr::new(path.to_str().expect("URLs received by the server should be valid unicode")).percent_encode().as_ref().as_str());
    Redirect::to(vue_root)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(database::stage())
        .mount("/", routes![redirect_root_to_web])
        .mount("/", routes![favicon])
        .mount("/test/", routes![login, hello])
        .mount("/api/", schema::routes())
        .mount("/web/", routes![singlepage, redirect_to_singlepage])
        .mount("/assets/", FileServer::from("../client/dist/assets"))
}
