use crate::db::get_db_connection;
use ::entity::ip_service;
use sea_orm::{
    prelude::DateTimeWithTimeZone, ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter,
    QueryTrait, Set, TryIntoModel,
};

pub struct Mutation;
pub struct Query;

impl Mutation {
    pub async fn create_ip_service(
        active_model: ip_service::ActiveModel,
    ) -> anyhow::Result<ip_service::Model> {
        let db = get_db_connection().await?;
        let model = active_model.save(&db).await?.try_into_model()?;

        Ok(model)
    }

    pub async fn update_ip_service(
        id: i64,
        model: ip_service::Model,
    ) -> anyhow::Result<ip_service::Model> {
        let db = get_db_connection().await?;
        let model = ip_service::ActiveModel {
            id: Set(id),
            ..model.into()
        }
        .save(&db)
        .await?
        .try_into_model()?;

        Ok(model)
    }

    pub async fn delete_ip_service(id: i64) -> anyhow::Result<bool> {
        let db = get_db_connection().await?;
        ip_service::ActiveModel {
            id: Set(id),
            ..Default::default()
        }
        .delete(&db)
        .await?;

        Ok(true)
    }
}

impl Query {
    pub async fn find_ip_service_by_id(id: i64) -> anyhow::Result<Option<ip_service::Model>> {
        let db = get_db_connection().await?;
        let model = ip_service::Entity::find_by_id(id).one(&db).await?;

        Ok(model)
    }

    pub async fn find_ip_service_by_port_older_then(
        port: i16,
        date: Option<DateTimeWithTimeZone>,
    ) -> anyhow::Result<Option<ip_service::Model>> {
        let db = get_db_connection().await?;
        let model = ip_service::Entity::find()
            .filter(ip_service::Column::Port.eq(port))
            .apply_if(date, |query, date| {
                query.filter(ip_service::Column::CreatedAt.lt(date))
            })
            .one(&db)
            .await?;

        Ok(model)
    }

    // pub async fn find_ip_service_by_ip(ip: &String) -> anyhow::Result<Option<ip_service::Model>> {
    //     let db = get_db_connection().await?;
    //     let model = ip_service::Entity::find()
    //         .filter(ip_service::Column::IpAddress.contains(ip))
    //         .one(&db)
    //         .await?;

    //     Ok(model)
    // }
}
