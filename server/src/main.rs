mod database;
mod schema;
mod orm;
mod betting;
mod realtime;

#[macro_use] extern crate rocket;

use rocket::http::CookieJar;
use rocket::http::RawStr;
use rocket::serde::json::Json;
use rocket::fs::{FileServer, NamedFile};
use rocket::response::Redirect;
use std::path::PathBuf;
use rocket_db_pools::Connection;
use crate::database::Db;
use crate::schema::AppError;

#[get("/hello")]
fn hello(cookies: &CookieJar<'_>) -> String {
    match cookies.get("username") {
        None => "You are not logged in.".to_string(),
        Some(username) => format!("You are logged in as {}", username),
    }
}

#[post("/login", data="<username>")]
async fn login(db: Connection<Db>, username: String, cookies: &CookieJar<'_>) -> Result<Option<Json<schema::User>>, AppError> {
    match orm::get_user(db, &username).await? {
        None => Ok(None),
        Some(user) => {
            cookies.add(("token", user.id.to_string()));
            Ok(Some(Json(user)))
        }
    }
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

#[get("/<path..>")]
fn redirect_to_singlepage(path: PathBuf) -> Redirect {
    let mut vue_root = String::from(FRONTEND_ROOT);
    vue_root.push_str("?nav_to="); // TODO: compile-time concat
    vue_root.push_str(RawStr::new(path.to_str().expect("URLs received by the server should be valid unicode")).percent_encode().as_ref().as_str());
    Redirect::to(vue_root)
}

use crate::realtime::ConnectedClients;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(database::stage())
        .manage(ConnectedClients::new())
        .mount("/", routes![redirect_root_to_web])
        .mount("/", routes![favicon])
        .mount("/test/", routes![realtime::stream, realtime::new_event])
        .mount("/api/", routes![login])
        .mount("/api/", schema::routes())
        .mount("/web/", routes![singlepage, redirect_to_singlepage])
        .mount("/assets/", FileServer::from("../client/dist/assets"))
}
