use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20241127_174046_create_table_schedule::Schedule, m20241127_174141_create_table_group::Group,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ScheduleGroup::Table)
                    .if_not_exists()
                    .col(pk_auto(ScheduleGroup::Id))
                    .col(unsigned(ScheduleGroup::ScheduleId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-schedulegroup-schedule")
                            .from(ScheduleGroup::Table, ScheduleGroup::ScheduleId)
                            .to(Schedule::Table, Schedule::Id),
                    )
                    .col(unsigned(ScheduleGroup::GroupId))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-schedulegroup-group")
                            .from(ScheduleGroup::Table, ScheduleGroup::GroupId)
                            .to(Group::Table, Group::Id),
                    )
                    .col(boolean(ScheduleGroup::Write))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ScheduleGroup::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum ScheduleGroup {
    Table,
    Id,
    ScheduleId,
    GroupId,
    Write,
}
