use entity::ip_main;
use migration::db::get_db_connection;
use sea_orm::{prelude::DateTimeWithTimeZone, ColumnTrait, EntityTrait, QueryFilter, QueryTrait};

pub struct Query;

impl Query {
    pub async fn find_ip_main_by_id(id: i64) -> anyhow::Result<Option<ip_main::Model>> {
        let db = get_db_connection().await?;
        let model = ip_main::Entity::find_by_id(id).one(&db).await?;

        Ok(model)
    }

    pub async fn find_ip_main_by_ip_older_then(
        ip: &str,
        date: Option<DateTimeWithTimeZone>,
    ) -> anyhow::Result<Option<ip_main::Model>> {
        let db = get_db_connection().await?;
        let model = ip_main::Entity::find()
            .filter(ip_main::Column::IpAddress.eq(ip))
            .apply_if(date, |query, date| {
                query.filter(ip_main::Column::CreatedAt.gt(date))
            })
            .one(&db)
            .await?;

        Ok(model)
    }
}
