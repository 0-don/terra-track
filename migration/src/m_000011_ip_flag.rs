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
                    .table(IpFlag::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(IpFlag::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(IpFlag::IpMainId).big_integer())
                    .col(ColumnDef::new(IpFlag::Img).text())
                    .col(ColumnDef::new(IpFlag::Emoji).text())
                    .col(ColumnDef::new(IpFlag::EmojiUnicode).text())
                    .col(
                        ColumnDef::new(IpFlag::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)),
                    )
                    .col(
                        ColumnDef::new(IpFlag::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_ip_flag_ip_main")
                            .from(IpFlag::Table, IpFlag::IpMainId)
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
            .drop_table(Table::drop().table(IpFlag::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum IpFlag {
    Table,
    Id,
    IpMainId,
    Img,
    Emoji,
    EmojiUnicode,
    CreatedAt,
    UpdatedAt,
}
