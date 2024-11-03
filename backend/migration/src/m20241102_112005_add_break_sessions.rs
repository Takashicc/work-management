use sea_orm_migration::{prelude::*, schema::*};

use crate::m20241102_105555_add_work_session::WorkSessions;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(BreakSessions::Table)
                    .if_not_exists()
                    .col(pk_auto(BreakSessions::Id))
                    .col(integer(BreakSessions::WorkSessionId))
                    .col(timestamp(BreakSessions::StartTime))
                    .col(timestamp(BreakSessions::EndTime).null())
                    .col(
                        timestamp(BreakSessions::CreatedAt)
                            .default(SimpleExpr::Custom("CURRENT_TIMESTAMP".into())),
                    )
                    .col(
                        timestamp(BreakSessions::UpdatedAt)
                            .default(SimpleExpr::Custom("CURRENT_TIMESTAMP".into()))
                            .extra("ON UPDATE CURRENT_TIMESTAMP"),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-break_sessions-work_session_id")
                            .from(BreakSessions::Table, BreakSessions::WorkSessionId)
                            .to(WorkSessions::Table, WorkSessions::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(BreakSessions::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum BreakSessions {
    Table,
    Id,
    WorkSessionId,
    StartTime,
    EndTime,
    CreatedAt,
    UpdatedAt,
}
