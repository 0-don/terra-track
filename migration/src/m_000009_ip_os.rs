use crate::m_000002_ip_main::IpMain;
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
                    .table(IpOs::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(IpOs::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(IpOs::IpMainId).big_integer().not_null())
                    .col(ColumnDef::new(IpOs::Fingerprint).text().not_null())
                    .col(ColumnDef::new(IpOs::Name).string().not_null())
                    .col(ColumnDef::new(IpOs::Cpe).string())
                    .col(ColumnDef::new(IpOs::Osfamily).string().not_null())
                    .col(ColumnDef::new(IpOs::Type).string().not_null())
                    .col(ColumnDef::new(IpOs::Vendor).string().not_null())
                    .col(ColumnDef::new(IpOs::OsGen).string())
                    .col(ColumnDef::new(IpOs::CpuArch).string())
                    .col(
                        ColumnDef::new(IpOs::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_ip_os_ip_main")
                            .from(IpOs::Table, IpOs::IpMainId)
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
            .drop_table(Table::drop().table(IpOs::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum IpOs {
    Table,
    Id,
    IpMainId,
    Fingerprint,
    Name,
    Cpe,
    Osfamily,
    Type,
    Vendor,
    OsGen,
    CpuArch,
    CreatedAt,
}
