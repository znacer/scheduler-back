pub use sea_orm_migration::prelude::*;

mod m20241127_174046_create_table_schedule;
mod m20241127_174053_create_table_category;
mod m20241127_174103_create_table_task;
mod m20241127_174141_create_table_group;
mod m20241127_174148_create_table_user;
mod m20241127_174159_create_table_user_group;
mod m20241127_174205_create_table_schedule_group;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20241127_174046_create_table_schedule::Migration),
            Box::new(m20241127_174053_create_table_category::Migration),
            Box::new(m20241127_174141_create_table_group::Migration),
            Box::new(m20241127_174103_create_table_task::Migration),
            Box::new(m20241127_174148_create_table_user::Migration),
            Box::new(m20241127_174159_create_table_user_group::Migration),
            Box::new(m20241127_174205_create_table_schedule_group::Migration),
        ]
    }
}
