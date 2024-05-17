mod database;
mod schema;
mod orm;

#[macro_use] extern crate rocket;

use rocket::http::CookieJar;
use rocket::fs::FileServer;
use rocket::response::Redirect;


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

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(database::stage())
        .mount("/test/", routes![login])
        .mount("/test/", routes![hello])
        .mount("/api/", routes![schema::create_user])
        .mount("/api/", routes![schema::get_all_users])
        .mount("/", FileServer::from("../client/dist"))
}
