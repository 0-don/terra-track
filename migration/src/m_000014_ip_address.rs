use crate::m_000013_ip_metadata::IpMetadata;
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
                    .table(IpAddress::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(IpAddress::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(IpAddress::IpMetadataId).integer())
                    .col(ColumnDef::new(IpAddress::Addr).text())
                    .col(ColumnDef::new(IpAddress::AddrType).text())
                    .col(
                        ColumnDef::new(IpAddress::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)),
                    )
                    .col(
                        ColumnDef::new(IpAddress::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_ip_address_ip_metadata")
                            .from(IpAddress::Table, IpAddress::IpMetadataId)
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
            .drop_table(Table::drop().table(IpAddress::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum IpAddress {
    Table,
    Id,
    IpMetadataId,
    Addr,
    AddrType,
    CreatedAt,
    UpdatedAt,
}
