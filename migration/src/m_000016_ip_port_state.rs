use crate::m_000015_ip_port::IpPort;
use sea_orm_migration::prelude::*;
use sea_query::{Keyword, SimpleExpr};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(IpPortState::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(IpPortState::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(IpPortState::IpPortId).integer())
                    .col(ColumnDef::new(IpPortState::State).text())
                    .col(ColumnDef::new(IpPortState::Reason).text())
                    .col(ColumnDef::new(IpPortState::ReasonTtl).text())
                    .col(
                        ColumnDef::new(IpPortState::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)),
                    )
                    .col(
                        ColumnDef::new(IpPortState::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_ip_port_state_ip_port")
                            .from(IpPortState::Table, IpPortState::IpPortId)
                            .to(IpPort::Table, IpPort::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(IpPortState::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum IpPortState {
    Table,
    Id,
    IpPortId,
    State,
    Reason,
    ReasonTtl,
    CreatedAt,
    UpdatedAt,
}
