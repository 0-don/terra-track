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
                    .table(IpSecurityFlags::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(IpSecurityFlags::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(IpSecurityFlags::IpMainId).integer())
                    .col(ColumnDef::new(IpSecurityFlags::SpamDatabase).text())
                    .col(ColumnDef::new(IpSecurityFlags::AdultHosting).boolean())
                    .col(ColumnDef::new(IpSecurityFlags::Hackers).boolean())
                    .col(ColumnDef::new(IpSecurityFlags::OpenProxy).boolean())
                    .col(
                        ColumnDef::new(IpSecurityFlags::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)),
                    )
                    .col(
                        ColumnDef::new(IpSecurityFlags::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_ip_security_flags_ip_main")
                            .from(IpSecurityFlags::Table, IpSecurityFlags::IpMainId)
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
            .drop_table(Table::drop().table(IpSecurityFlags::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum IpSecurityFlags {
    Table,
    Id,
    IpMainId,
    SpamDatabase,
    AdultHosting,
    Hackers,
    OpenProxy,
    CreatedAt,
    UpdatedAt,
}
