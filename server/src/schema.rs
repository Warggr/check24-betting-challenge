use rocket::response::status::Created;
use rocket::serde::{Serialize, Deserialize};
use rocket::serde::json::Json;
use rocket_db_pools::Connection;

use crate::database::Db;
use crate::orm;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub name: String,
    pub points: i32,
    pub community: i32, // TODO: is it one or multiple communities?
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Community {
    pub name: String,
}

/* #[delete("/<id>")]
async fn delete(mut db: Connection<Db>, id: i64) -> Result<Option<()>> {
    let result = sqlx::query!("DELETE FROM posts WHERE id = ?", id)
        .execute(&mut **db)
        .await?;

    Ok((result.rows_affected() == 1).then(|| ()))
} */

#[post("/user", data = "<username>")]
pub async fn create_user(db: Connection<Db>, username: String) -> orm::Result<Created<()>> {
    orm::create_user(db, User { name: username }).await?;
    Ok(Created::new("/"))
}

#[get("/user/all")]
pub async fn get_all_users(db: Connection<Db>) -> orm::Result<Json<Vec<User>>> {
    let users = orm::get_all_users(db).await?;
    Ok(Json(users))
}
