use crate::m_000002_ip_main::IpMain;
use sea_orm_migration::{
    prelude::*,
    sea_orm::{EnumIter, Iterable},
    sea_query::extension::postgres::Type,
};
use sea_query::{Keyword, SimpleExpr};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden, EnumIter)]
pub enum ServiceProtocol {
    Table,
    IP,
    TCP,
    UDP,
    SCTP,
}

#[derive(Iden, EnumIter)]
pub enum ServiceConf {
    Table,
    #[iden = "0"]
    ZERO,
    #[iden = "1"]
    ONE,
    #[iden = "2"]
    TWO,
    #[iden = "3"]
    THREE,
    #[iden = "4"]
    FOUR,
    #[iden = "5"]
    FIVE,
    #[iden = "6"]
    SIX,
    #[iden = "7"]
    SEVEN,
    #[iden = "8"]
    EIGHT,
    #[iden = "9"]
    NINE,
    #[iden = "10"]
    TEN,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(ServiceProtocol::Table)
                    .values(ServiceProtocol::iter().skip(1))
                    .to_owned(),
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(ServiceConf::Table)
                    .values(ServiceConf::iter().skip(1))
                    .to_owned(),
            )
            .await?;

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
                    .col(
                        ColumnDef::new(IpService::Protocol)
                            .enumeration(ServiceProtocol::Table, ServiceProtocol::iter().skip(1))
                            .default(SimpleExpr::Value(ServiceProtocol::TCP.to_string().into()))
                            .not_null(),
                    )
                    .col(ColumnDef::new(IpService::Port).small_unsigned().not_null())
                    .col(ColumnDef::new(IpService::Name).string().not_null())
                    .col(
                        ColumnDef::new(IpService::Conf)
                            .enumeration(ServiceConf::Table, ServiceConf::iter().skip(1))
                            .not_null(),
                    )
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
