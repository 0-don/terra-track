use crate::m_000002_ip_main::IpMain;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(IpNetworkDetails::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(IpNetworkDetails::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(IpNetworkDetails::IpMainId).big_integer())
                    .col(ColumnDef::new(IpNetworkDetails::Asn).big_integer())
                    .col(ColumnDef::new(IpNetworkDetails::Org).text())
                    .col(ColumnDef::new(IpNetworkDetails::Isp).text())
                    .col(ColumnDef::new(IpNetworkDetails::Domain).text())
                    .col(ColumnDef::new(IpNetworkDetails::PtrRecord).text())
                    .col(ColumnDef::new(IpNetworkDetails::AsnNumber).big_integer())
                    .col(ColumnDef::new(IpNetworkDetails::AsnName).text())
                    .col(ColumnDef::new(IpNetworkDetails::IpRange).text())
                    .col(
                        ColumnDef::new(IpNetworkDetails::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_ip_network_details_ip_main")
                            .from(IpNetworkDetails::Table, IpNetworkDetails::IpMainId)
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
            .drop_table(Table::drop().table(IpNetworkDetails::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum IpNetworkDetails {
    Table,
    Id,
    IpMainId,
    Asn,
    Org,
    Isp,
    Domain,
    PtrRecord,
    AsnNumber,
    AsnName,
    IpRange,
    CreatedAt,
}
