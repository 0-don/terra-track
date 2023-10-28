use ::entity::ip_main;
use sea_orm::{ActiveModelTrait, DbConn, EntityTrait, Set, TryIntoModel};

pub struct Mutation;
pub struct Query;

impl Mutation {
    pub async fn create_ip_main(
        db: &DbConn,
        active_model: ip_main::ActiveModel,
    ) -> anyhow::Result<ip_main::Model> {
        let model = active_model.save(db).await?.try_into_model()?;

        Ok(model)
    }

    pub async fn update_ip_main(
        db: &DbConn,
        id: i32,
        model: ip_main::Model,
    ) -> anyhow::Result<ip_main::Model> {
        let model = ip_main::ActiveModel {
            id: Set(id),
            ..model.into()
        }
        .save(db)
        .await?
        .try_into_model()?;

        Ok(model)
    }

    pub async fn delete_ip_main(db: &DbConn, id: i32) -> anyhow::Result<bool> {
        ip_main::ActiveModel {
            id: Set(id),
            ..Default::default()
        }
        .delete(db)
        .await?;

        Ok(true)
    }
}

impl Query {
    pub async fn find_note_by_id(db: &DbConn, id: i32) -> anyhow::Result<Option<ip_main::Model>> {
        let model = ip_main::Entity::find_by_id(id).one(db).await?;

        Ok(model)
    }
}
