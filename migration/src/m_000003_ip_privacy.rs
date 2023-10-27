use sea_orm_migration::prelude::*;
use sea_query::{ForeignKey, ForeignKeyAction};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(IpPrivacy::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(IpPrivacy::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(IpPrivacy::IpMainId).integer())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_ip_privacy_ip_main")
                            .from(IpPrivacy::Table, IpPrivacy::IpMainId)
                            .to(IpMain::Table, IpMain::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(IpPrivacy::IsTor).boolean())
                    .col(ColumnDef::new(IpPrivacy::IsProxy).boolean())
                    .col(ColumnDef::new(IpPrivacy::IsVpn).boolean())
                    .col(ColumnDef::new(IpPrivacy::IsAbuser).boolean())
                    .col(ColumnDef::new(IpPrivacy::IsRelay).boolean())
                    .col(ColumnDef::new(IpPrivacy::IsHosting).boolean())
                    .col(ColumnDef::new(IpPrivacy::Service).text())
                    .col(
                        ColumnDef::new(IpPrivacy::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)),
                    )
                    .col(
                        ColumnDef::new(IpPrivacy::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(IpPrivacy::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum IpPrivacy {
    Table,
    Id,
    IpMainId,
    IsTor,
    IsProxy,
    IsVpn,
    IsAbuser,
    IsRelay,
    IsHosting,
    Service,
    CreatedAt,
    UpdatedAt,
}

// Ensure IpMain is accessible and properly imported
use crate::m_000002_ip_main::IpMain;
