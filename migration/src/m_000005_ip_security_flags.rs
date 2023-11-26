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
                    .table(IpSecurityFlags::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(IpSecurityFlags::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(IpSecurityFlags::IpMainId).big_integer())
                    // Security risk related columns
                    .col(
                        ColumnDef::new(IpSecurityFlags::IsSpamDatabase)
                            .boolean()
                            .default(false)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(IpSecurityFlags::IsAdultHosting)
                            .boolean()
                            .default(false)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(IpSecurityFlags::IsHackers)
                            .boolean()
                            .default(false)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(IpSecurityFlags::IsOpenProxy)
                            .boolean()
                            .default(false)
                            .not_null(),
                    )
                    // Privacy related columns
                    .col(
                        ColumnDef::new(IpSecurityFlags::IsTor)
                            .boolean()
                            .default(false)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(IpSecurityFlags::IsProxy)
                            .boolean()
                            .default(false)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(IpSecurityFlags::IsVpn)
                            .boolean()
                            .default(false)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(IpSecurityFlags::IsAbuser)
                            .boolean()
                            .default(false)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(IpSecurityFlags::IsRelay)
                            .boolean()
                            .default(false)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(IpSecurityFlags::IsHosting)
                            .boolean()
                            .default(false)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(IpSecurityFlags::IsBogon)
                            .boolean()
                            .default(false)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(IpSecurityFlags::IsMobile)
                            .boolean()
                            .default(false)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(IpSecurityFlags::IsDatacenter)
                            .boolean()
                            .default(false)
                            .not_null(),
                    )
                    .col(ColumnDef::new(IpSecurityFlags::Service).text())
                    .col(
                        ColumnDef::new(IpSecurityFlags::CreatedAt)
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
    // Security risk related columns
    IsSpamDatabase,
    IsAdultHosting,
    IsHackers,
    IsOpenProxy,
    // Privacy related columns
    IsTor,
    IsProxy,
    IsVpn,
    IsAbuser,
    IsRelay,
    IsHosting,
    IsBogon,
    IsMobile,
    IsDatacenter,
    Service,
    CreatedAt,
}
