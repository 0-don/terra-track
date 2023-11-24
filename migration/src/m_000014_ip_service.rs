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
                    .table(IpService::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(IpService::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(IpService::IpMainId).big_integer().not_null())
                    .col(ColumnDef::new(IpService::Protocol).string().not_null())
                    .col(ColumnDef::new(IpService::Port).small_integer().not_null())
                    .col(ColumnDef::new(IpService::Name).string())
                    .col(ColumnDef::new(IpService::Conf).string())
                    .col(ColumnDef::new(IpService::Version).string())
                    .col(ColumnDef::new(IpService::Product).string())
                    .col(ColumnDef::new(IpService::ExtraInfo).string())
                    .col(ColumnDef::new(IpService::Tunnel).string())
                    .col(ColumnDef::new(IpService::Proto).string())
                    .col(ColumnDef::new(IpService::Rpcnum).string())
                    .col(ColumnDef::new(IpService::Lowver).string())
                    .col(ColumnDef::new(IpService::Highver).string())
                    .col(ColumnDef::new(IpService::Hostname).string())
                    .col(ColumnDef::new(IpService::Method).string().not_null())
                    .col(ColumnDef::new(IpService::OsType).string())
                    .col(ColumnDef::new(IpService::CpuArch).string())
                    .col(ColumnDef::new(IpService::DeviceType).string())
                    .col(ColumnDef::new(IpService::ServiceFP).string())
                    .col(ColumnDef::new(IpService::Cpe).string())
                    .col(
                        ColumnDef::new(IpService::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_ip_service_ip_main")
                            .from(IpService::Table, IpService::IpMainId)
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
            .drop_table(Table::drop().table(IpService::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum IpService {
    Table,
    Id,
    IpMainId,
    Protocol,
    Port,
    Name,
    Conf,
    Version,
    Product,
    ExtraInfo,
    Tunnel,
    Proto,
    Rpcnum,
    Lowver,
    Highver,
    Hostname,
    OsType,
    CpuArch,
    DeviceType,
    ServiceFP,
    Cpe,
    Method,
    CreatedAt,
}
