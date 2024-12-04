use actix_web::{http::header::Header, Error, HttpRequest};
use actix_web_httpauth::headers::{self, authorization::Bearer};
use base64::prelude::*;
use itertools::Itertools;
use log::debug;
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection, DbErr};
use serde::{Deserialize, Serialize};
use serde_json;
use std::env;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct IdRequest {
    pub id: i32,
}
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

pub fn token_username(req: &HttpRequest) -> Result<String, Error> {
    let token = headers::authorization::Authorization::<Bearer>::parse(req)?;
    let token = token.as_ref().token();
    let data = token.split(".").collect_vec()[1];
    let data = data.to_string() + "==";

    debug!("{}", data);
    let output = BASE64_STANDARD.decode(data).unwrap();
    let output = String::from_utf8(output).unwrap();
    let output: serde_json::Value = serde_json::from_str(&output)?;
    let output = output["name"].to_string().replace("\"", "");

    debug!("TEST: {}", output);
    Ok(output)
}
