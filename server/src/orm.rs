use rocket_db_pools::Connection;
use rocket::futures::TryStreamExt;
use rocket::futures::TryFutureExt;
use rocket::{Request, Response};
use rocket::http::Status;
use rocket::response::Responder;
use sqlx::Error;
use sqlx::sqlite::SqliteRow;
use crate::database::Db;
use crate::schema::{Community, CommunityId, User, UserId};

pub type Result<T> = std::result::Result<T, Error>;

pub async fn create_user(mut db: Connection<Db>, user: User) -> Result<UserId> {
    assert_eq!(user.id, 0);
    sqlx::query!("INSERT INTO users (name, points) VALUES (?, ?)", user.name, user.points)
        .execute(&mut **db)
        .map_ok(|result| convert(result.last_insert_rowid()))
        .await
}

pub async fn get_user(mut db: Connection<Db>, user_name: String) -> Result<Option<User>> {
    return sqlx::query_as!(User, "SELECT * FROM users WHERE name = (?)", user_name)
        .fetch_optional(&mut **db)
        .await
}

pub async fn delete_user(mut db: Connection<Db>, user_name: String) -> Result<()> {
    sqlx::query!("DELETE FROM users WHERE name = (?)", user_name)
        .execute(&mut **db)
        .map_ok(|_| ())
        .await
}

pub async fn list_users(mut db: Connection<Db>) -> Result<Vec<User>> {
    sqlx::query_as!(User, "SELECT id, name, points FROM users")
        .fetch(&mut **db)
        .try_collect::<Vec<_>>()
        .await
}

pub async fn create_community(mut db: Connection<Db>, community: Community) -> Result<CommunityId> {
    sqlx::query!("INSERT INTO communities (name) VALUES (?)", community.name)
        .execute(&mut **db)
        .map_ok(|result| convert(result.last_insert_rowid()))
        .await
}

pub async fn add_to_community(mut db: Connection<Db>, community_id: CommunityId, user_id: UserId) -> Result<()> {
    sqlx::query!("INSERT INTO users_x_communities (user_id, community_id) VALUES (?, ?)", user_id, community_id)
        .execute(&mut **db)
        .map_ok(|_| ())
        .await
}

fn convert<Precision: TryFrom<i64>>(n : i64) -> Precision {
    Precision::try_from(n).ok().expect("SQL value does not fit into i32")
}

pub async fn get_users_of_community(mut db: Connection<Db>, community_id: CommunityId) -> Result<Vec<User>> {
    sqlx::query_as!(
            User,
            "SELECT u.* FROM users u JOIN users_x_communities x ON x.user_id = u.id WHERE x.community_id = (?)",
            community_id
        )
        .fetch_all(&mut **db)
        .await
}
