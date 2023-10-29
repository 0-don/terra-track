use crate::db::get_db_connection;
use ::entity::ip_main;
use sea_orm::{ActiveModelTrait, EntityTrait, Set, TryIntoModel};

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

    pub async fn update_ip_main(id: i32, model: ip_main::Model) -> anyhow::Result<ip_main::Model> {
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

    pub async fn delete_ip_main(id: i32) -> anyhow::Result<bool> {
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
    pub async fn find_ip_main_by_id(id: i32) -> anyhow::Result<Option<ip_main::Model>> {
        let db = get_db_connection().await?;
        let model = ip_main::Entity::find_by_id(id).one(&db).await?;

        Ok(model)
    }
}
