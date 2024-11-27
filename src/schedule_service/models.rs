use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use utoipa::ToSchema;

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Clone, Default, ToSchema, FromRow)]
pub struct Task {
    pub(super) id: i64,
    pub(super) name: String,
    pub(super) start: i64,
    pub(super) duration: i64,
    pub(super) description: String,
    pub(super) category: i32,
    pub(super) schedule_id: i64,
}

impl Task {
    pub fn example() -> Self {
        Task {
            id: 1,
            name: "example".to_string(),
            start: 1729468800000,
            duration: 3_600_000,
            description: "".to_string(),
            category: 0,
            schedule_id: 1,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema, FromRow, Default)]
pub struct Schedule {
    pub(super) id: i64,
    pub(super) name: String,
    pub(super) description: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema, FromRow, Default)]
pub struct Group {
    pub(super) id: i64,
    pub(super) name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema, FromRow, Default)]
pub struct User {
    pub(super) id: i64,
    pub(super) name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema, FromRow, Default)]
pub struct UserGroup {
    pub(super) id: i64,
    pub(super) user_id: i64,
    pub(super) group_id: i64,
    pub(super) admin: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema, FromRow, Default)]
pub struct ScheduleGroup {
    pub(super) id: i64,
    pub(super) schedule_id: i64,
    pub(super) group_id: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, ToSchema, FromRow, Default)]
pub struct GroupRights {
    pub(super) id: i64,
    pub(super) group_id: i64,
    pub(super) schedule_id: i64,
    pub(super) write: bool,
}
