use sea_orm_migration::{prelude::*, schema::*};

use crate::m20220101_000001_add_projects::Projects;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(WorkSessions::Table)
                    .if_not_exists()
                    .col(pk_auto(WorkSessions::Id))
                    .col(integer(WorkSessions::ProjectId))
                    .col(timestamp(WorkSessions::StartTime))
                    .col(timestamp(WorkSessions::EndTime).null())
                    .col(
                        timestamp(WorkSessions::CreatedAt)
                            .default(SimpleExpr::Custom("CURRENT_TIMESTAMP".into())),
                    )
                    .col(
                        timestamp(WorkSessions::UpdatedAt)
                            .default(SimpleExpr::Custom("CURRENT_TIMESTAMP".into()))
                            .extra("ON UPDATE CURRENT_TIMESTAMP"),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-work_sessions-project_id")
                            .from(WorkSessions::Table, WorkSessions::ProjectId)
                            .to(Projects::Table, Projects::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(WorkSessions::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum WorkSessions {
    Table,
    Id,
    ProjectId,
    StartTime,
    EndTime,
    CreatedAt,
    UpdatedAt,
}
