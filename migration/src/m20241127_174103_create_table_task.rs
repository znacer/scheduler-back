use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20241127_174046_create_table_schedule::Schedule, m20241127_174053_create_table_category::Category
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Task::Table)
                    .if_not_exists()
                    .col(pk_auto(Task::Id))
                    .col(string(Task::Name))
                    .col(big_unsigned(Task::Start))
                    .col(big_unsigned(Task::Duration))
                    .col(text(Task::Description))
                    .col(unsigned(Task::CategoryId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-task-category")
                            .from(Task::Table, Task::CategoryId)
                            .to(Category::Table, Category::Id),
                    )
                    .col(unsigned(Task::ScheduleId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-task-schedule")
                            .from(Task::Table, Task::ScheduleId)
                            .to(Schedule::Table, Schedule::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Task::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Task {
    Table,
    Id,
    Name,
    Start,
    Duration,
    Description,
    CategoryId,
    ScheduleId,
}
