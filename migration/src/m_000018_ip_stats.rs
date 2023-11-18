use crate::m_000013_ip_metadata::IpMetadata;
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
                    .table(IpStats::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(IpStats::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(IpStats::IpMetadataId).integer())
                    .col(ColumnDef::new(IpStats::Up).text())
                    .col(ColumnDef::new(IpStats::Down).text())
                    .col(ColumnDef::new(IpStats::Total).text())
                    .col(
                        ColumnDef::new(IpStats::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)),
                    )
                    .col(
                        ColumnDef::new(IpStats::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_ip_stats_ip_metadata")
                            .from(IpStats::Table, IpStats::IpMetadataId)
                            .to(IpMetadata::Table, IpMetadata::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(IpStats::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum IpStats {
    Table,
    Id,
    IpMetadataId,
    Up,
    Down,
    Total,
    CreatedAt,
    UpdatedAt,
}
