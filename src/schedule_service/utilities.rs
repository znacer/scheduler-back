use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection, DbErr};
use std::env;

pub async fn sql_connect() -> DatabaseConnection {
    let username = match env::var_os("POSTGRES_USER") {
        Some(val) => match val.into_string() {
            Ok(val) => val,
            _ => "postgres".to_string(),
        },
        _ => "postgres".to_string(),
    };

    let password = match env::var_os("POSTGRES_PASSWORD") {
        Some(val) => match val.into_string() {
            Ok(val) => val,
            _ => "mysecretpassword".to_string(),
        },
        _ => "mysecretpassword".to_string(),
    };

    let address = match env::var_os("POSTGRES_HOST") {
        Some(val) => match val.into_string() {
            Ok(val) => val,
            _ => "localhost".to_string(),
        },
        _ => "localhost".to_string(),
    };

    let database = match env::var_os("POSTGRES_DATABASE") {
        Some(val) => match val.into_string() {
            Ok(val) => val,
            _ => "public".to_string(),
        },
        _ => "public".to_string(),
    };

    let connection_string = format!("postgres://{username}:{password}@{address}/{database}");
    Database::connect(connection_string.as_str()).await.unwrap()
}

pub(super) async fn create_tables() -> Result<(), DbErr> {
    let db = sql_connect().await;
    Migrator::up(&db, None).await
}
