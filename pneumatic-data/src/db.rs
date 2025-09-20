use std::path::Path;

use sqlx::{Sqlite, SqlitePool, migrate::MigrateDatabase, sqlite::SqlitePoolOptions};

pub type Db = SqlitePool;

#[tracing::instrument]
pub async fn init_db(path: &Path) -> SqlitePool {
    let mut path = path.to_owned();
    match std::fs::create_dir_all(path.clone()) {
        Ok(_) => {}
        Err(err) => {
            panic!("error creating directory {}", err);
        }
    }

    path.push("db2.sqlite");

    tracing::info!("Creating database at: {}", path.to_str().unwrap());
    Sqlite::create_database(
        format!(
            "sqlite:{}",
            path.to_str().expect("path should be something")
        )
        .as_str(),
    )
    .await
    .expect("failed to create database");

    let db = SqlitePoolOptions::new()
        .connect(path.to_str().unwrap())
        .await
        .unwrap();

    sqlx::migrate!("./migrations").run(&db).await.unwrap();

    db
}
