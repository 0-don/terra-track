use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(IpMain::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(IpMain::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(IpMain::IpAddress)
                            .text()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(IpMain::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)),
                    )
                    .col(
                        ColumnDef::new(IpMain::UpdatedAt)
                            .timestamp_with_time_zone()
                            .default(SimpleExpr::Keyword(Keyword::CurrentTimestamp)),
                    )
                    .index(
                        Index::create()
                            .name("idx_ip_main_ip_address")
                            .table(IpMain::Table)
                            .col(IpMain::IpAddress)
                            .unique(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .exec_stmt(
                Query::insert()
                    .into_table(IpMain::Table)
                    .columns([IpMain::IpAddress])
                    .values_panic(["1.0.0.0".into()])
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(IpMain::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub(crate) enum IpMain {
    Table,
    Id,
    IpAddress,
    CreatedAt,
    UpdatedAt,
}
