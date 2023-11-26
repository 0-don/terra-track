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
                    .table(IpHostScript::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(IpHostScript::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(IpHostScript::IpMainId).big_integer().not_null())
                    .col(ColumnDef::new(IpHostScript::Key).string().not_null())
                    .col(ColumnDef::new(IpHostScript::Value).json().not_null())
                    .col(
                        ColumnDef::new(IpHostScript::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_ip_os_ip_main")
                            .from(IpHostScript::Table, IpHostScript::IpMainId)
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
            .drop_table(Table::drop().table(IpHostScript::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum IpHostScript {
    Table,
    Id,
    IpMainId,
    Key,
    Value,
    CreatedAt,
}
