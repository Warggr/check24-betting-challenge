use rocket::{Build, fairing, Rocket};
use rocket::fairing::AdHoc;
use rocket_db_pools::Database;
use rocket_db_pools::sqlx;
use sqlx::migrate::Migrator;
use std::path::Path;
use log::error;

#[derive(Database)]
#[database("db")]
pub struct Db(sqlx::SqlitePool);

async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    // TODO OPTIMIZE: this is very ugly
    match Db::fetch(&rocket) {
        Some(db) => match Migrator::new(Path::new("./migrations")).await {
            Ok(mig) => match mig.run(&**db).await {
                Ok(_) => Ok(rocket),
                Err(e) => {
                    error!("Failed to initialize SQLx database: {}", e);
                    Err(rocket)
                },
            }
            Err(e) => {
                error!("Failed to migrate: {}", e);
                Err(rocket)
            }
        },
        None => {
            error!("Could not initialize database");
            Err(rocket)
        },
    }
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("SQLx Stage", |rocket| async {
        rocket.attach(Db::init())
            .attach(AdHoc::try_on_ignite("SQLx Migrations", run_migrations))
    })
}
