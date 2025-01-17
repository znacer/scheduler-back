//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.1

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(
    Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize, ToSchema, Default,
)]
#[schema(as = User)]
#[sea_orm(table_name = "group")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::schedule_group::Entity")]
    ScheduleGroup,
    #[sea_orm(has_many = "super::user_group::Entity")]
    UserGroup,
}

impl Related<super::schedule_group::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ScheduleGroup.def()
    }
}

impl Related<super::user_group::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::UserGroup.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
