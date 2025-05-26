use sea_orm_migration::prelude::*;

mod m20231201_000001_create_users_table;
mod m20231201_000002_create_notes_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20231201_000001_create_users_table::Migration),
            Box::new(m20231201_000002_create_notes_table::Migration),
        ]
    }
}