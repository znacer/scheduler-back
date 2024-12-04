use actix_web::error::{Error, ResponseError};
use sea_orm::{DbErr, SqlErr, SqlxError, SqlxPostgresError};
use strum::Display;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError, Display)]
pub enum ProjectError {
    Actix(#[from] Error),
    DbErr(DbErr),
    SqlErr(#[from] SqlErr),
    SqlxError(#[from] SqlxError),
    SqlxPostgressError(#[from] SqlxPostgresError),
}

impl From<DbErr> for ProjectError {
    fn from(value: DbErr) -> Self {
        ProjectError::DbErr(value)
    }
}
// impl From<Box<dyn ResponseError>> for ProjectError {
//     fn from(value: Box<dyn ResponseError>) -> Self {
//         ProjectError::Actix(value.into())
//     }
// }
impl ResponseError for ProjectError {}
