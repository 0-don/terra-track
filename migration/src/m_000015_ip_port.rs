use sea_orm_migration::prelude::*;
use sea_query::{Keyword, SimpleExpr};

use crate::m_000014_ip_address::IpAddress;

// Ensure IpAddress is accessible and properly imported
// You might need to adjust this import based on where IpAddress is defined.

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(IpPort::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(IpPort::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(IpPort::IpAddressId).integer())
                    .col(ColumnDef::new(IpPort::Protocol).text())
                    .col(ColumnDef::new(IpPort::PortId).text())
                    .col(ColumnDef::new(IpPort::State).text())
                    .col(
                        ColumnDef::new(IpPort::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)),
                    )
                    .col(
                        ColumnDef::new(IpPort::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_ip_port_ip_address")
                            .from(IpPort::Table, IpPort::IpAddressId)
                            .to(IpAddress::Table, IpAddress::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(IpPort::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum IpPort {
    Table,
    Id,
    IpAddressId,
    Protocol,
    PortId,
    State,
    CreatedAt,
    UpdatedAt,
}
