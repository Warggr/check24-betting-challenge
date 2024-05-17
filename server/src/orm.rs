use rocket_db_pools::Connection;
use rocket::futures::TryStreamExt;
use rocket::futures::TryFutureExt;
use crate::database::Db;
use crate::schema::User;

pub type Result<T> = std::result::Result<T, String>;

pub async fn create_user(mut db: Connection<Db>, user: User) -> Result<()> {
    sqlx::query!("INSERT INTO users (username) VALUES (?)", user.name)
        .execute(&mut **db)
        .map_ok(|_| ())
        .map_err(|err| err.to_string())
        .await
}

pub async fn get_user(mut db: Connection<Db>, user_name: String) -> Result<Option<User>> {
    return sqlx::query!("SELECT * FROM users WHERE username = (?)", user_name)
        .fetch(&mut **db)
        .try_collect::<Vec<_>>()
        .map_ok(|r : Vec<_>|
            Some( User { name: r.into_iter().next()?.username })
        )
        .await
        .map_err(|err| err.to_string())
}

pub async fn delete_user(mut db: Connection<Db>, user_name: String) -> Result<()> {
    sqlx::query!("DELETE FROM users WHERE username = (?)", user_name)
        .execute(&mut **db)
        .map_ok(|_| ())
        .map_err(|err| err.to_string())
        .await
}

pub async fn get_all_users(mut db: Connection<Db>) -> Result<Vec<User>> {
    sqlx::query!("SELECT username FROM users")
        .fetch(&mut **db)
        .map_ok(|record| User{ name: record.username })
        .try_collect::<Vec<_>>()
        .await
        .map_err(|err| err.to_string())
}
