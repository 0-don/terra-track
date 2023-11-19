use crate::db::get_db_connection;
use ::entity::ip_main;
use sea_orm::{
    prelude::DateTimeWithTimeZone, ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter,
    QueryTrait, Set, TryIntoModel,
};

pub struct Mutation;
pub struct Query;

impl Mutation {
    pub async fn create_ip_main(
        active_model: ip_main::ActiveModel,
    ) -> anyhow::Result<ip_main::Model> {
        let db = get_db_connection().await?;
        let model = active_model.save(&db).await?.try_into_model()?;

        Ok(model)
    }

    pub async fn upsert_ip_main(
        active_model: ip_main::ActiveModel,
    ) -> anyhow::Result<ip_main::Model> {
        let db = get_db_connection().await?;
        let mut model = ip_main::Entity::find()
            .filter(ip_main::Column::IpAddress.eq(active_model.ip_address.as_ref().to_owned()))
            .one(&db)
            .await?
            .map(|model| model.try_into_model())
            .transpose()?;

        if model.is_none() {
            model = active_model.insert(&db).await.ok();
        }

        Ok(model.unwrap())
    }

    pub async fn update_ip_main(id: i64, model: ip_main::Model) -> anyhow::Result<ip_main::Model> {
        let db = get_db_connection().await?;
        let model = ip_main::ActiveModel {
            id: Set(id),
            ..model.into()
        }
        .save(&db)
        .await?
        .try_into_model()?;

        Ok(model)
    }

    pub async fn delete_ip_main(id: i64) -> anyhow::Result<bool> {
        let db = get_db_connection().await?;
        ip_main::ActiveModel {
            id: Set(id),
            ..Default::default()
        }
        .delete(&db)
        .await?;

        Ok(true)
    }
}

impl Query {
    pub async fn find_ip_main_by_id(id: i64) -> anyhow::Result<Option<ip_main::Model>> {
        let db = get_db_connection().await?;
        let model = ip_main::Entity::find_by_id(id).one(&db).await?;

        Ok(model)
    }

    pub async fn find_ip_main_by_ip_older_then(
        ip: &String,
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
