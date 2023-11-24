use crate::{m_000002_ip_main::IpMain, m_000015_ip_service::IpService};
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
                    .table(IpServiceScript::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(IpServiceScript::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(IpServiceScript::IpMainId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(IpServiceScript::IpServiceId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(IpServiceScript::Key).string().not_null())
                    .col(ColumnDef::new(IpServiceScript::Value).json().not_null())
                    .col(
                        ColumnDef::new(IpServiceScript::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_ip_service_ip_main")
                            .from(IpServiceScript::Table, IpServiceScript::IpMainId)
                            .to(IpMain::Table, IpMain::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_ip_service_ip_service")
                            .from(IpServiceScript::Table, IpServiceScript::IpServiceId)
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
            .drop_table(Table::drop().table(IpServiceScript::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum IpServiceScript {
    Table,
    Id,
    IpMainId,
    IpServiceId,
    Key,
    Value,
    CreatedAt,
}
