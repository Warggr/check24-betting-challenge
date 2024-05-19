use rocket::response::Responder;
use rocket::response::status::{Created, NoContent};
use rocket::serde::{Serialize, Deserialize};
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use sqlx::{Error, FromRow};

use crate::database::Db;
use crate::orm;

const MAX_COMMUNITIES_PER_USER: usize = 5;
pub type CommunityId = i64;
pub type UserId = i64;

#[derive(Debug, Clone, Deserialize, Serialize, FromRow)]
#[serde(crate = "rocket::serde")]
pub struct User {
    pub id: UserId,
    pub name: String,
    pub points: i64,
}

#[derive(Clone, Deserialize, Serialize, FromRow)]
#[serde(crate = "rocket::serde")]
pub struct Community {
    pub id: CommunityId,
    pub name: String,
}

type GameId = i64;

#[derive(PartialEq)]
pub struct GameResult {
    pub home: u16,
    pub away: u16,
}

pub struct Bet {
    pub game_id: GameId,
    pub result: GameResult,
}

/* #[delete("/<id>")]
async fn delete(mut db: Connection<Db>, id: i64) -> Result<Option<()>> {
    let result = sqlx::query!("DELETE FROM posts WHERE id = ?", id)
        .execute(&mut **db)
        .await?;

    Ok((result.rows_affected() == 1).then(|| ()))
} */

#[derive(Responder)]
pub struct AppError {
    message: String,
}

impl From<sqlx::Error> for AppError {
    fn from(value: sqlx::Error) -> Self {
        AppError { message: value.to_string() }
    }
}

type Result<T> = std::result::Result<T, AppError>;

#[post("/user", data = "<username>")]
pub async fn create_user(db: Connection<Db>, username: String) -> Result<Created<()>> {
    let new_user = User { id: 0, name: username, points: 0 };
    let id = orm::create_user(db, new_user).await?;
    Ok(Created::new(format!("/user/{}", id)))
}

#[get("/user/all")]
pub async fn list_users(db: Connection<Db>) -> Result<Json<Vec<User>>> {
    let users = orm::list_users(db).await?;
    Ok(Json(users))
}

#[post("/community", data = "<name>")]
pub async fn create_community(db: Connection<Db>, name: String) -> Result<Created<()>> {
    let new_community = Community { id: 0, name };
    let id = orm::create_community(db, new_community).await?;
    Ok(Created::new(format!("/community/{}", id)))
}

#[put("/community/<community_id>/users/<user_id>")]
pub async fn add_user_to_community(db: Connection<Db>, community_id: CommunityId, user_id: UserId) -> Result<NoContent> {
    // TODO check whether user has already 5 communities
    orm::add_to_community(db, community_id, user_id).await?;
    Ok(NoContent)
}

#[get("/community/<community_id>/users")]
pub async fn get_community_users(db: Connection<Db>, community_id: CommunityId) -> Result<Json<Vec<User>>> {
    let users = orm::get_users_of_community(db, community_id).await?;
    Ok(Json(users))
}

pub fn routes() -> Vec<rocket::Route> {
    routes![create_user, list_users, create_community, add_user_to_community, get_community_users]
}
