use sea_orm_migration::{prelude::*, sea_orm::{EnumIter, DeriveActiveEnum}};
use sea_query::{Keyword, SimpleExpr};

use crate::m_000002_ip_main::IpMain;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[derive(EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "ProtocolEnum")]
pub enum ProtocolEnum {
    #[sea_orm(string_value = "TCP")]
    TCP,
    #[sea_orm(string_value = "UDP")]
    UDP,
}

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
                    .col(ColumnDef::new(IpService::IpMainId).big_integer())
                    .col(ColumnDef::new(IpService::Port).small_integer())
                    .col(ColumnDef::new(IpService::Name).text())
                    .col(ColumnDef::new(IpService::Product).text())
                    .col(ColumnDef::new(IpService::Version).text())
                    .col(ColumnDef::new(IpService::ExtraInfo).text())
                    .col(ColumnDef::new(IpService::OsType).text())
                    .col(ColumnDef::new(IpService::Method).text())
                    .col(ColumnDef::new(IpService::Conf).text())
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
                            .name("fk_ip_hosting_details_ip_main")
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
    Port,
    Name,
    Product,
    Version,
    ExtraInfo,
    OsType,
    Method,
    Conf,
    CreatedAt,
    UpdatedAt,
}
