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
                    .table(IpOrganization::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(IpOrganization::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(IpOrganization::IpMainId).integer())
                    .col(ColumnDef::new(IpOrganization::Asn).integer())
                    .col(ColumnDef::new(IpOrganization::Domain).text())
                    .col(ColumnDef::new(IpOrganization::OrgName).text())
                    .col(ColumnDef::new(IpOrganization::Network).text())
                    .col(ColumnDef::new(IpOrganization::Route).text())
                    .col(ColumnDef::new(IpOrganization::Type).text())
                    .col(
                        ColumnDef::new(IpOrganization::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)),
                    )
                    .col(
                        ColumnDef::new(IpOrganization::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_ip_organization_ip_main")
                            .from(IpOrganization::Table, IpOrganization::IpMainId)
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
            .drop_table(Table::drop().table(IpOrganization::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum IpOrganization {
    Table,
    Id,
    IpMainId,
    Asn,
    Domain,
    OrgName,
    Network,
    Route,
    Type,
    CreatedAt,
    UpdatedAt,
}
