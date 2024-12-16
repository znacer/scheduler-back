use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Schedule::Table)
                    .if_not_exists()
                    .col(pk_auto(Schedule::Id))
                    .col(string(Schedule::Name))
                    .col(text(Schedule::Description))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Schedule::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Schedule {
    Table,
    Id,
    Name,
    Description,
}
