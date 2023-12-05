use entity::ip_service_script;
use migration::db::get_db_connection;
use sea_orm::EntityTrait;

pub struct Query;

impl Query {
    pub async fn find_ip_service_script_by_id(
        id: i64,
    ) -> anyhow::Result<Option<ip_service_script::Model>> {
        let db = get_db_connection().await?;
        let model = ip_service_script::Entity::find_by_id(id).one(&db).await?;

        Ok(model)
    }

    // pub async fn find_ip_service_by_port_and_ip_main_id_older_then(
    //     port: i16,
    //     ip_main_id: i64,
    //     date: Option<DateTimeWithTimeZone>,
    // ) -> anyhow::Result<Option<ip_service::Model>> {
    //     let db = get_db_connection().await?;
    //     let model = ip_service::Entity::find()
    //         .filter(ip_service::Column::Port.eq(port))
    //         .filter(ip_service::Column::IpMainId.eq(ip_main_id))
    //         .apply_if(date, |query, date| {
    //             query.filter(ip_service::Column::CreatedAt.lt(date))
    //         })
    //         .one(&db)
    //         .await?;

    //     Ok(model)
    // }

    // pub async fn find_ip_service_by_ip(ip: &String) -> anyhow::Result<Option<ip_service::Model>> {
    //     let db = get_db_connection().await?;
    //     let model = ip_service::Entity::find()
    //         .filter(ip_service::Column::IpAddress.contains(ip))
    //         .one(&db)
    //         .await?;

    //     Ok(model)
    // }
}
