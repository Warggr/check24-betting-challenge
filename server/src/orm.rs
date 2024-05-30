use rocket_db_pools::Connection;
use rocket::futures::TryFutureExt;
use sqlx::Error;
use crate::database::Db;
use crate::schema::{CommunityId, /*Game,*/ User, UserId};

pub type Result<T> = std::result::Result<T, Error>;

pub async fn create_user(mut db: Connection<Db>, user: User) -> Result<UserId> {
    assert_eq!(user.id, 0);
    sqlx::query!("INSERT INTO users (name, points) VALUES (?, ?)", user.name, user.points)
        .execute(&mut **db)
        .map_ok(|result| convert(result.last_insert_rowid()))
        .await
}

pub async fn get_user(mut db: Connection<Db>, user_name: &String) -> Result<Option<User>> {
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

pub async fn add_to_community(mut db: Connection<Db>, community_id: CommunityId, user_id: UserId) -> Result<()> {
    sqlx::query!("INSERT INTO users_x_communities (user_id, community_id) VALUES (?, ?)", user_id, community_id)
        .execute(&mut **db)
        .map_ok(|_| ())
        .await
}

pub fn convert<Precision: TryFrom<i64>>(n : i64) -> Precision {
    Precision::try_from(n).ok().expect("SQL value does not fit into i32")
}

/*pub async fn list_games(mut db: Connection<Db>) -> Result<Vec<Game>> {
    sqlx::query_as!(Game, "SELECT * FROM games")
        .fetch_all(&mut **db)
        .await
}*/
