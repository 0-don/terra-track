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
                    .table(IpTimes::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(IpTimes::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(IpTimes::IpMetadataId).integer())
                    .col(ColumnDef::new(IpTimes::Srtt).text())
                    .col(ColumnDef::new(IpTimes::Rttvar).text())
                    .col(ColumnDef::new(IpTimes::To).text())
                    .col(
                        ColumnDef::new(IpTimes::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)),
                    )
                    .col(
                        ColumnDef::new(IpTimes::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_ip_times_ip_metadata")
                            .from(IpTimes::Table, IpTimes::IpMetadataId)
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
            .drop_table(Table::drop().table(IpTimes::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum IpTimes {
    Table,
    Id,
    IpMetadataId,
    Srtt,
    Rttvar,
    To,
    CreatedAt,
    UpdatedAt,
}
