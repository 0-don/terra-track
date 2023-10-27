use crate::m_000002_ip_main::IpMain;
use sea_orm_migration::prelude::*;
use sea_query::{ForeignKey, ForeignKeyAction, Keyword, SimpleExpr};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(IpMetadata::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(IpMetadata::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(IpMetadata::IpMainId).integer())
                    .col(ColumnDef::new(IpMetadata::Scanner).text())
                    .col(ColumnDef::new(IpMetadata::Args).text())
                    .col(ColumnDef::new(IpMetadata::Start).text())
                    .col(ColumnDef::new(IpMetadata::Version).text())
                    .col(ColumnDef::new(IpMetadata::XmlOutputVersion).text())
                    .col(ColumnDef::new(IpMetadata::ScanInfo).text())
                    .col(ColumnDef::new(IpMetadata::Verbose).text())
                    .col(ColumnDef::new(IpMetadata::Debugging).text())
                    .col(ColumnDef::new(IpMetadata::RunStats).text())
                    .col(
                        ColumnDef::new(IpMetadata::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)),
                    )
                    .col(
                        ColumnDef::new(IpMetadata::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_ip_metadata_ip_main")
                            .from(IpMetadata::Table, IpMetadata::IpMainId)
                            .to(IpMain::Table, IpMain::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(IpMetadata::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum IpMetadata {
    Table,
    Id,
    IpMainId,
    Scanner,
    Args,
    Start,
    Version,
    XmlOutputVersion,
    ScanInfo,
    Verbose,
    Debugging,
    RunStats,
    CreatedAt,
    UpdatedAt,
}
