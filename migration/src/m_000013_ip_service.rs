use sea_orm_migration::{
    prelude::*,
    sea_orm::{EnumIter, Iterable},
    sea_query::extension::postgres::Type,
};
use sea_query::{Keyword, SimpleExpr};

use crate::m_000002_ip_main::IpMain;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(Iden, EnumIter)]
pub enum Protocol {
    Table,
    #[iden = "TCP"]
    TCP,
    #[iden = "UPD"]
    UDP,
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(Protocol::Table)
                    .values(Protocol::iter().skip(1))
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
                            .enumeration(Protocol::Table, Protocol::iter().skip(1))
                            .default(SimpleExpr::Value(Protocol::TCP.to_string().into()))
                            .not_null(),
                    )
                    .col(ColumnDef::new(IpService::Port).small_integer().not_null())
                    .col(ColumnDef::new(IpService::Name).string().not_null())
                    .col(ColumnDef::new(IpService::Product).string())
                    .col(ColumnDef::new(IpService::ServiceFp).text())
                    .col(ColumnDef::new(IpService::Version).string())
                    .col(ColumnDef::new(IpService::ExtraInfo).string())
                    .col(ColumnDef::new(IpService::Method).string().not_null())
                    .col(ColumnDef::new(IpService::OsType).string())
                    .col(ColumnDef::new(IpService::CpuArch).string())
                    .col(
                        ColumnDef::new(IpService::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)),
                    )
                    .col(
                        ColumnDef::new(IpService::UpdatedAt)
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
    Product,
    Version,
    ExtraInfo,
    ServiceFp,
    Method,
    OsType,
    CpuArch,
    CreatedAt,
    UpdatedAt,
}
