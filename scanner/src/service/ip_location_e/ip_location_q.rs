use ::entity::ip_location;
use migration::db::get_db_connection;
use sea_orm::{prelude::DateTimeWithTimeZone, ColumnTrait, EntityTrait, QueryFilter, QueryTrait};

pub struct Query;

impl Query {
    pub async fn find_ip_location_by_id(id: i64) -> anyhow::Result<Option<ip_location::Model>> {
        let db = get_db_connection().await?;
        let model = ip_location::Entity::find_by_id(id).one(&db).await?;

        Ok(model)
    }

    pub async fn find_ip_location_by_ip_main_id_older_then(
        ip_main_id: i64,
        time_ago: Option<DateTimeWithTimeZone>,
    ) -> anyhow::Result<Option<ip_location::Model>> {
        let db = get_db_connection().await?;
        let model = ip_location::Entity::find()
            .filter(ip_location::Column::IpMainId.eq(ip_main_id))
            .apply_if(time_ago, |query, date| {
                query.filter(ip_location::Column::CreatedAt.lt(date))
            })
            .one(&db)
            .await?;

        Ok(model)
    }

    // pub async fn find_ip_location_by_ip(ip: &String) -> anyhow::Result<Option<ip_location::Model>> {
    //     let db = get_db_connection().await?;
    //     let model = ip_location::Entity::find()
    //         .filter(ip_location::Column::IpAddress.contains(ip))
    //         .one(&db)
    //         .await?;

    //     Ok(model)
    // }
}
