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
                    .table(IpConnection::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(IpConnection::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(IpConnection::IpMainId).integer())
                    .col(ColumnDef::new(IpConnection::Asn).integer())
                    .col(ColumnDef::new(IpConnection::Org).text())
                    .col(ColumnDef::new(IpConnection::Isp).text())
                    .col(ColumnDef::new(IpConnection::Domain).text())
                    .col(
                        ColumnDef::new(IpConnection::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)),
                    )
                    .col(
                        ColumnDef::new(IpConnection::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_ip_connection_ip_main")
                            .from(IpConnection::Table, IpConnection::IpMainId)
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
            .drop_table(Table::drop().table(IpConnection::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum IpConnection {
    Table,
    Id,
    IpMainId,
    Asn,
    Org,
    Isp,
    Domain,
    CreatedAt,
    UpdatedAt,
}
