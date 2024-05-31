use rocket::response::Responder;
use rocket::response::status::{Created, NoContent};
use rocket::serde::{Serialize, Deserialize};
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use sqlx::FromRow;
use rocket::http::{CookieJar, Status};
//use rocket::http::hyper::header::CACHE_CONTROL;
use rocket::fs::NamedFile;
use rocket::futures::TryFutureExt;

use crate::database::Db;
use crate::orm;
use crate::betting;
use crate::betting::{GameResult, Winner};

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

pub(crate) type GameId = i64;

#[derive(Serialize, Deserialize, FromRow)]
#[serde(crate = "rocket::serde")]
pub struct Game {
    pub id: GameId,
    pub team_home: String,
    pub team_away: String,
    pub starts_at: String, // TODO: this should be a date
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
    #[response(status=500)]
    InternalError(String),
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
    let cookie = cookies.get("token").ok_or(AppError::HttpError(Status::Unauthorized))?;
    let user_id = cookie.value().parse::<UserId>().map_err(|_| AppError::HttpError(Status::Unauthorized))?;

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
#[get("/games.v0")]
pub async fn list_games(mut db: Connection<Db>) -> Result<Json<Vec<Game>>> {
    let games = sqlx::query_as!(Game, "SELECT team_home, team_away, id, starts_at FROM games")
        .fetch_all(&mut **db)
        .await?;
    Ok(Json(games))
    /* Response::build_from(Json(games).into())
        // maximum possible age of 1year, see https://stackoverflow.com/a/25201898
        .raw_header(CACHE_CONTROL.as_str(), "public, max-age=31536000, immutable")
        .finalize()
        .into()*/
}

#[get("/games.v0")]
pub async fn list_games_from_file() -> Option<NamedFile> {
    NamedFile::open("./data/games.json").await.ok()
}

#[derive(PartialEq, Serialize, Deserialize)]
struct GameResultWithGameId {
    pub game_id: i64,
    pub home: i64,
    pub away: i64,
}

#[get("/results")]
pub async fn list_results(mut db: Connection<Db>) -> Result<Json<Vec<GameResultWithGameId>>> {
    let results = sqlx::query!("SELECT id, home_goals AS home, away_goals AS away FROM games WHERE home_goals IS NOT NULL AND away_goals IS NOT NULL")
        .fetch_all(&mut **db)
        .await?
        .into_iter()
        .map(|row| -> Option<GameResultWithGameId> {
            Some(GameResultWithGameId { game_id: row.id, home: row.home?, away: row.away? })
        })
        .collect::<Option<Vec<GameResultWithGameId>>>()
        .ok_or(AppError::InternalError("Could not parse Result".to_string()))
        ?;
    Ok(Json(results))
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

/*
pub async fn group_points(mut db: Connection<Db>) -> Result<Vec<(i64, i64)>> {
    let groups = sqlx::query!("SELECT points, COUNT(1) AS count FROM users GROUP BY points")
        .fetch_all(&mut **db)
        .await?;
    return groups.iter().map(|row| (row.points, row.count)).collect();
}


#[get("/groups/points")]
pub async fn group_points_api(db: Connection<Db>) -> Result<Json<Map<String, i64>>> {
    group_points(db).await.map_ok(|points| Json(points))
}

pub async fn group_winner_bet(mut db: Connection<Db>, game_id: GameId, points: i64) -> Result<Vec<(Winner, i64)>> {
    let groups = sqlx::query!("
SELECT
    (bet_team_home > bet_team_away) AS home_winner,
    (bet_team_home < bet_team_away) AS away_winner,
    COUNT(1) AS count
FROM users u
JOIN bets on u.id = user_id
WHERE u.game_id = ? AND points = ?
GROUP BY home_winner, away_winner;", game_id, points)
        .fetch_all(&mut **db)
        .await?;
    return groups.iter().map(|row| (if row.home_winner { Winner::Home } else if row.away_winner { Winner::Away } else {Winner::Draw}, row.count )).collect();
}

pub async fn group_diff_bet(mut db: Connection<Db>, game_id: GameId, points: i64, winner: Winner) -> Result<Vec<(i64, i64)>> {
    let groups = sqlx::query!("
SELECT
    (bet_team_home > bet_team_away) AS home_winner,
    (bet_team_home < bet_team_away) AS away_winner,
    (bet_team_home - bet_team_away) AS goal_diff,
    COUNT(1) AS count
FROM users u
JOIN bets on u.id = user_id
WHERE u.game_id = ? AND points = ? AND home_winner = ? AND away_winner = ?
GROUP BY goal_diff;", game_id, points, (winner == Winner::Home), (winner == Winner::Away))
        .fetch_all(&mut **db)
        .await?;
    return groups.iter().map(|row| (row.goal_diff, row.count )).collect();
}

pub async fn group_exact_bet(mut db: Connection<Db>, game_id: GameId, points: i64, winner: Winner, diff : i64) -> Result<Vec<(GameResult, i64)>> {
    let groups = sqlx::query!("
SELECT
    bet_team_home, bet_team_away,
    (bet_team_home - bet_team_away) AS goal_diff,
    COUNT(1) AS count
FROM users u
JOIN bets on u.id = user_id
WHERE u.game_id = ? AND points = ? AND goal_diff = ?
GROUP BY bet_team_home, bet_team_away;", game_id, points, diff)
        .fetch_all(&mut **db)
        .await?;
    return groups.iter().map(|row| (GameResult { home: row.bet_team_home, away: row.bet_team_away }, row.count )).collect();
}

pub fn fixResult(db: Connection<Db>, game_id : GameId, result: GameResult) {
    for (point, _) in group_points(db) {
        for (winner, _) in group_winner_bet(db, game_id, point) {
            if winner != betting::winner(result) {
                // give 0 points
                continue
            }
            for (goal_diff, _) in group_diff_bet(db, game_id, point, winner) {
                if goal_diff != betting::goal_difference(result) {
                    sqlx::query("UPDATE users SET points = points + 4 WHERE TODO:winner").execute().await;
                    continue
                }
                for (score, _) in group_exact_bet(db, game_id, point, winner, goal_diff) {
                    if score != result {
                        sqlx::query("UPDATE users SET points = points + 6 WHERE TODO:winner").execute().await;
                        continue
                    }
                    sqlx::query("UPDATE users SET points = points + 8 WHERE team_home").execute().await;
                }
            }
        }
    }
}
 */

pub fn routes() -> Vec<rocket::Route> {
    routes![
        create_user, create_community, add_user_to_community,
        get_users, count_users, get_user_rank,
        get_community_users, count_community_users, get_community_user_rank,
        get_user_communities, list_games, list_results,
        create_bet, get_bets,
//        group_points_api
    ]
}
