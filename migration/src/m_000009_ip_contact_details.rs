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
                    .table(IpContactDetails::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(IpContactDetails::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(IpContactDetails::IpMainId).integer())
                    .col(ColumnDef::new(IpContactDetails::ContactName).text())
                    .col(ColumnDef::new(IpContactDetails::ContactAddress).text())
                    .col(ColumnDef::new(IpContactDetails::Phone).text())
                    .col(ColumnDef::new(IpContactDetails::Remarks).text())
                    .col(
                        ColumnDef::new(IpContactDetails::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)),
                    )
                    .col(
                        ColumnDef::new(IpContactDetails::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_ip_contact_details_ip_main")
                            .from(IpContactDetails::Table, IpContactDetails::IpMainId)
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
            .drop_table(Table::drop().table(IpContactDetails::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum IpContactDetails {
    Table,
    Id,
    IpMainId,
    ContactName,
    ContactAddress,
    Phone,
    Remarks,
    CreatedAt,
    UpdatedAt,
}
