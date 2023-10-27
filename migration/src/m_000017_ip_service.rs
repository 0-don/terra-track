use sea_orm_migration::prelude::*;
use sea_query::{Keyword, SimpleExpr};

use crate::m_000015_ip_port::IpPort;

// Ensure IpPort is accessible and properly imported
// You might need to adjust this import based on where IpPort is defined.

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(IpService::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(IpService::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(IpService::IpPortId).integer())
                    .col(ColumnDef::new(IpService::Name).text())
                    .col(ColumnDef::new(IpService::Product).text())
                    .col(ColumnDef::new(IpService::Version).text())
                    .col(ColumnDef::new(IpService::ExtraInfo).text())
                    .col(ColumnDef::new(IpService::OsType).text())
                    .col(ColumnDef::new(IpService::Method).text())
                    .col(ColumnDef::new(IpService::Conf).text())
                    .col(
                        ColumnDef::new(IpService::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)),
                    )
                    .col(
                        ColumnDef::new(IpService::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_ip_service_ip_port")
                            .from(IpService::Table, IpService::IpPortId)
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
            .drop_table(Table::drop().table(IpService::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum IpService {
    Table,
    Id,
    IpPortId,
    Name,
    Product,
    Version,
    ExtraInfo,
    OsType,
    Method,
    Conf,
    CreatedAt,
    UpdatedAt,
}
