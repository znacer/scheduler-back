use sea_orm_migration::{prelude::*, schema::*};

use crate::{m20241127_174141_create_table_group::Group, m20241127_174148_create_table_user::User};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserGroup::Table)
                    .if_not_exists()
                    .col(pk_auto(UserGroup::Id))
                    .col(unsigned(UserGroup::UserId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-usergroup-user")
                            .from(UserGroup::Table, UserGroup::UserId)
                            .to(User::Table, User::Id),
                    )
                    .col(unsigned(UserGroup::GroupId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-usergroup-group")
                            .from(UserGroup::Table, UserGroup::GroupId)
                            .to(Group::Table, Group::Id),
                    )
                    .col(boolean(UserGroup::Admin))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserGroup::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum UserGroup {
    Table,
    Id,
    UserId,
    GroupId,
    Admin,
}
