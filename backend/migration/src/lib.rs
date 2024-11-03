pub use sea_orm_migration::prelude::*;

mod m20220101_000001_add_projects;
mod m20241102_105555_add_work_session;
mod m20241102_112005_add_break_sessions;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_add_projects::Migration),
            Box::new(m20241102_105555_add_work_session::Migration),
            Box::new(m20241102_112005_add_break_sessions::Migration),
        ]
    }
}
