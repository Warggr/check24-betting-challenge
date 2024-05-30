use rocket::response::Responder;
use rocket::response::status::{Created, NoContent};
use rocket::serde::{Serialize, Deserialize};
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use sqlx::FromRow;
use rocket::http::{CookieJar, Status};
//use rocket::http::hyper::header::CACHE_CONTROL;
use rocket::fs::NamedFile;

use crate::database::Db;
use crate::orm;
use crate::schema::AppError::HttpError;

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

/*#[derive(Serialize, Deserialize, FromRow)]
#[serde(crate = "rocket::serde")]
pub struct Game {
    pub id: GameId,
    pub team_home: String,
    pub team_away: String,
    pub starts_at: DateTime<Utc>,
}*/

#[derive(PartialEq, Serialize, Deserialize)]
pub struct GameResult {
    pub home: u16,
    pub away: u16,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct Bet {
    pub user_id: UserId,
    pub game_id: GameId,
    pub bet_team_home: i64,
    pub bet_team_away: i64,
}

#[derive(Responder)]
pub enum AppError {
    #[response(status=400)]
    SqlError(String),
    HttpError(Status),
}

impl From<sqlx::Error> for AppError {
    fn from(value: sqlx::Error) -> Self {
        AppError::SqlError(value.to_string())
    }
}

type Result<T> = std::result::Result<T, AppError>;

#[post("/users", data = "<username>")]
pub async fn create_user(db: Connection<Db>, username: String) -> Result<Created<()>> {
    let new_user = User { id: 0, name: username, points: 0 };
    let id = orm::create_user(db, new_user).await?;
    Ok(Created::new(format!("/user/{}", id)))
}

#[post("/communities", data = "<name>")]
pub async fn create_community(mut db: Connection<Db>, name: String, cookies: &CookieJar<'_>) -> Result<Created<()>> {
    let cookie = cookies.get("token").ok_or(HttpError(Status::Unauthorized))?;
    let user_id = cookie.value().parse::<UserId>().map_err(|_| HttpError(Status::Unauthorized))?;

    let community_id = sqlx::query!("INSERT INTO communities (name) VALUES (?)", name)
        .execute(&mut **db)
        .await
        .map(|result| orm::convert(result.last_insert_rowid()))?
        ;
    orm::add_to_community(db, community_id, user_id).await?;
    Ok(Created::new(format!("/community/{}", community_id)))
}

#[put("/communities/<community_id>/users/<user_id>")]
pub async fn add_user_to_community(db: Connection<Db>, community_id: CommunityId, user_id: UserId) -> Result<NoContent> {
    // TODO check whether user has already 5 communities
    orm::add_to_community(db, community_id, user_id).await?;
    Ok(NoContent)
}

#[get("/users?count")]
pub async fn count_users(mut db: Connection<Db>) -> Result<Json<i32>> {
    let nb_users = sqlx::query!("SELECT COUNT(1) AS count FROM users")
        .fetch_one(&mut **db)
        .await?
        .count;
    Ok(Json(nb_users))
}

#[get("/communities/<community_id>/users?count")]
pub async fn count_community_users(mut db: Connection<Db>, community_id: CommunityId) -> Result<Json<i32>> {
    let nb_users = sqlx::query!("SELECT COUNT(1) AS count FROM users u JOIN users_x_communities x ON x.user_id = u.id WHERE x.community_id = (?)", community_id)
        .fetch_one(&mut **db)
        .await?
        .count;
    Ok(Json(nb_users))
}

#[get("/users?rank&<u>")]
pub async fn get_user_rank(mut db: Connection<Db>, u: UserId) -> Result<Json<i64>> {
    let rank = sqlx::query!(
        "SELECT rank FROM (SELECT u.id AS id, ROW_NUMBER() OVER (ORDER BY points DESC) AS rank FROM users u) WHERE id = (?)"
        , u)
        .fetch_one(&mut **db)
        .await?
        .rank;
    Ok(Json(rank))
}

#[get("/communities/<community_id>/users?rank&<u>")]
pub async fn get_community_user_rank(mut db: Connection<Db>, community_id: CommunityId, u: UserId) -> Result<Json<i64>> {
    let rank = sqlx::query!(
        "SELECT rank FROM (SELECT u.id AS id, ROW_NUMBER() OVER (ORDER BY points DESC) AS rank FROM users u JOIN users_x_communities x ON x.user_id = u.id WHERE x.community_id = (?)) WHERE id = (?)"
        , community_id, u)
        .fetch_one(&mut **db)
        .await?
        .rank;
    Ok(Json(rank))
}

#[get("/users?page&<n>", rank=2)]
pub async fn get_users(mut db: Connection<Db>, n: i32) -> Result<Json<Vec<User>>> {
    let offset = 10 * n;
    let users = sqlx::query_as!(
            User,
            "SELECT u.* FROM users u ORDER BY points DESC LIMIT (?), 10",
            offset
        )
        .fetch_all(&mut **db)
        .await?;
    Ok(Json(users))
}

#[get("/communities/<community_id>/users?page&<n>", rank=2)]
pub async fn get_community_users(mut db: Connection<Db>, community_id: CommunityId, n: i32) -> Result<Json<Vec<User>>> {
    let offset = 10 * n;
    let users = sqlx::query_as!(
            User,
            "SELECT u.* FROM users u JOIN users_x_communities x ON x.user_id = u.id WHERE x.community_id = (?) ORDER BY points DESC LIMIT (?), 10",
            community_id, offset
        )
        .fetch_all(&mut **db)
        .await?;
    Ok(Json(users))
}

#[get("/user/<user_id>/communities")]
pub async fn get_user_communities(mut db: Connection<Db>, user_id: UserId) -> Result<Json<Vec<Community>>> {
    let communities = sqlx::query_as!(Community, "SELECT c.* FROM communities c JOIN users_x_communities x ON x.community_id = c.id WHERE x.user_id = (?)", user_id)
        .fetch_all(&mut **db)
        .await?;
    Ok(Json(communities))
}

// This can be cached forever
// In case we want to update it at some point, we use a version number (currently 0)
// TODO: once the players for the quarterfinals etc. are fixed, this will need to be updated
// we'll need to think about another pattern then -> maybe set a more correct max-age?
/*#[get("/games.v0")]
pub async fn list_games(db: Connection<Db>) -> Result<Response<'static>> {
    let games = orm::list_games(db).await?;
    Response::build_from(Json(games).into())
        // maximum possible age of 1year, see https://stackoverflow.com/a/25201898
        .raw_header(CACHE_CONTROL.as_str(), "public, max-age=31536000, immutable")
        .finalize()
        .into()
}*/

#[get("/games.v0")]
pub async fn list_games_from_file() -> Option<NamedFile> {
    NamedFile::open("./data/games.json").await.ok()
}

#[post("/user/<user_id>/bet/<game_id>", data="<bet>")]
pub async fn create_bet(mut db: Connection<Db>, user_id: UserId, game_id: GameId, bet: Json<GameResult>) -> Result<Created<()>> {
    sqlx::query!("INSERT INTO bets (user_id, game_id, bet_team_home, bet_team_away) VALUES (?, ?, ?, ?)", user_id, game_id, bet.home, bet.away)
        .execute(&mut **db)
        .await?;
    Ok(Created::new("/"))
}

#[get("/user/<user_id>/bets")]
pub async fn get_bets(mut db: Connection<Db>, user_id: UserId) -> Result<Json<Vec<Bet>>> {
    let bets = sqlx::query_as!(Bet, "SELECT * FROM bets WHERE user_id = ?", user_id)
        .fetch_all(&mut **db)
        .await?;
    Ok(Json(bets))
}

pub fn routes() -> Vec<rocket::Route> {
    routes![
        create_user, create_community, add_user_to_community,
        get_users, count_users, get_user_rank,
        get_community_users, count_community_users, get_community_user_rank,
        get_user_communities, list_games_from_file,
        create_bet, get_bets,
    ]
}
