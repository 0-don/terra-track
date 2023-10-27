use crate::m_000002_ip_main::IpMain;
use sea_orm_migration::prelude::*;
use sea_query::{ForeignKey, ForeignKeyAction, SimpleExpr};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(IpLocation::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(IpLocation::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(IpLocation::IpMainId).integer())
                    .col(ColumnDef::new(IpLocation::Continent).text())
                    .col(ColumnDef::new(IpLocation::Country).text())
                    .col(ColumnDef::new(IpLocation::CountryCode).text())
                    .col(ColumnDef::new(IpLocation::State).text())
                    .col(ColumnDef::new(IpLocation::City).text())
                    .col(ColumnDef::new(IpLocation::Latitude).float())
                    .col(ColumnDef::new(IpLocation::Longitude).float())
                    .col(ColumnDef::new(IpLocation::Postal).text())
                    .col(ColumnDef::new(IpLocation::Timezone).text())
                    .col(
                        ColumnDef::new(IpLocation::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)),
                    )
                    .col(
                        ColumnDef::new(IpLocation::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_ip_location_ip_main")
                            .from(IpLocation::Table, IpLocation::IpMainId)
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
            .drop_table(Table::drop().table(IpLocation::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum IpLocation {
    Table,
    Id,
    IpMainId,
    Continent,
    Country,
    CountryCode,
    State,
    City,
    Latitude,
    Longitude,
    Postal,
    Timezone,
    CreatedAt,
    UpdatedAt,
}
