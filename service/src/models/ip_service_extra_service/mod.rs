use crate::db::get_db_connection;
use entity::ip_service_extra;
use sea_orm::{ActiveModelTrait, EntityTrait, Set, TryIntoModel, QueryFilter, ColumnTrait};

pub struct Mutation;
pub struct Query;

impl Mutation {
    pub async fn create_ip_service_extra(
        active_model: ip_service_extra::ActiveModel,
    ) -> anyhow::Result<ip_service_extra::Model> {
        let db = get_db_connection().await?;
        let model = active_model.save(&db).await?.try_into_model()?;

        Ok(model)
    }

    pub async fn update_ip_service_extra(
        id: i64,
        model: ip_service_extra::Model,
    ) -> anyhow::Result<ip_service_extra::Model> {
        let db = get_db_connection().await?;
        let model = ip_service_extra::ActiveModel {
            id: Set(id),
            ..model.into()
        }
        .save(&db)
        .await?
        .try_into_model()?;

        Ok(model)
    }

    pub async fn delete_ip_service_extra(id: i64) -> anyhow::Result<bool> {
        let db = get_db_connection().await?;
        ip_service_extra::ActiveModel {
            id: Set(id),
            ..Default::default()
        }
        .delete(&db)
        .await?;

        Ok(true)
    }

    pub async fn delete_ip_service_extra_by_ip_service_id(
        ip_service_id: i64,
    ) -> anyhow::Result<bool> {
        let db = get_db_connection().await?;
        ip_service_extra::Entity::delete_many()
            .filter(ip_service_extra::Column::IpServiceId.eq(ip_service_id))
            .exec(&db)
            .await?;

        Ok(true)
    }
}

impl Query {
    pub async fn find_ip_service_extra_by_id(
        id: i64,
    ) -> anyhow::Result<Option<ip_service_extra::Model>> {
        let db = get_db_connection().await?;
        let model = ip_service_extra::Entity::find_by_id(id).one(&db).await?;

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
