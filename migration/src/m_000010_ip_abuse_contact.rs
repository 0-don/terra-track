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
                    .table(IpAbuseContact::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(IpAbuseContact::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(IpAbuseContact::IpMainId).big_integer())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_ip_abuse_contact_ip_main")
                            .from(IpAbuseContact::Table, IpAbuseContact::IpMainId)
                            .to(IpMain::Table, IpMain::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(IpAbuseContact::Address).text())
                    .col(ColumnDef::new(IpAbuseContact::Country).text())
                    .col(ColumnDef::new(IpAbuseContact::Email).text())
                    .col(ColumnDef::new(IpAbuseContact::Name).text())
                    .col(ColumnDef::new(IpAbuseContact::Network).text())
                    .col(ColumnDef::new(IpAbuseContact::Phone).text())
                    .col(
                        ColumnDef::new(IpAbuseContact::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(IpAbuseContact::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum IpAbuseContact {
    Table,
    Id,
    IpMainId,
    Address,
    Country,
    Email,
    Name,
    Network,
    Phone,
    CreatedAt,
}
