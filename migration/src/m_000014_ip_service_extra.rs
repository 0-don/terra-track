use sea_orm_migration::prelude::*;
use sea_query::{Keyword, SimpleExpr};

use crate::{m_000002_ip_main::IpMain, m_000013_ip_service::IpService};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(IpServiceExtra::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(IpServiceExtra::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(IpServiceExtra::IpMainId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(IpServiceExtra::IpServiceId)
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(IpServiceExtra::Key).string().not_null())
                    .col(ColumnDef::new(IpServiceExtra::Value).json().not_null())
                    .col(
                        ColumnDef::new(IpServiceExtra::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)),
                    )
                    .col(
                        ColumnDef::new(IpServiceExtra::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_ip_service_ip_main")
                            .from(IpServiceExtra::Table, IpServiceExtra::IpMainId)
                            .to(IpMain::Table, IpMain::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_ip_service_ip_service")
                            .from(IpServiceExtra::Table, IpServiceExtra::IpServiceId)
                            .to(IpService::Table, IpService::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(IpServiceExtra::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum IpServiceExtra {
    Table,
    Id,
    IpMainId,
    IpServiceId,
    Key,
    Value,
    CreatedAt,
    UpdatedAt,
}
