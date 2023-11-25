use crate::db::get_db_connection;
use ::entity::ip_host_script;
use sea_orm::{prelude::DateTimeWithTimeZone, ColumnTrait, EntityTrait, QueryFilter, QueryTrait};

pub struct Query;

impl Query {
    pub async fn find_host_script_by_id(id: i64) -> anyhow::Result<Option<ip_host_script::Model>> {
        let db = get_db_connection().await?;
        let model = ip_host_script::Entity::find_by_id(id).one(&db).await?;

        Ok(model)
    }

    pub async fn find_host_script_by_ip_main_id_older_then(
        ip_main_id: i64,
        time_ago: Option<DateTimeWithTimeZone>,
    ) -> anyhow::Result<Option<ip_host_script::Model>> {
        let db = get_db_connection().await?;
        let model = ip_host_script::Entity::find()
            .filter(ip_host_script::Column::IpMainId.eq(ip_main_id))
            .apply_if(time_ago, |query, date| {
                query.filter(ip_host_script::Column::CreatedAt.lt(date))
            })
            .one(&db)
            .await?;

        Ok(model)
    }

    // pub async fn find_ip_host_script_by_ip(ip: &String) -> anyhow::Result<Option<ip_host_script::Model>> {
    //     let db = get_db_connection().await?;
    //     let model = ip_host_script::Entity::find()
    //         .filter(ip_host_script::Column::IpAddress.contains(ip))
    //         .one(&db)
    //         .await?;

    //     Ok(model)
    // }
}
