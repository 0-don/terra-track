use crate::{db::get_db_connection, utils::convert_i32_to_ipv4_string};
use ::entity::scan_batch;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QueryOrder, Set, TryIntoModel,
};

pub struct Mutation;
pub struct Query;

const BATCH_SIZE: i32 = 10;

impl Mutation {
    pub async fn create_scan_batch(
        active_model: scan_batch::ActiveModel,
    ) -> anyhow::Result<scan_batch::Model> {
        let db = get_db_connection().await?;
        let model = active_model.save(&db).await?.try_into_model()?;

        Ok(model)
    }

    pub async fn update_scan_batch(
        model: scan_batch::ActiveModel,
    ) -> anyhow::Result<scan_batch::Model> {
        let db = get_db_connection().await?;
        let model = scan_batch::ActiveModel {
            id: model.id.clone(),
            ..model.into()
        }
        .save(&db)
        .await?
        .try_into_model()?;

        Ok(model)
    }

    pub async fn delete_scan_batch(id: i32) -> anyhow::Result<bool> {
        let db = get_db_connection().await?;
        scan_batch::ActiveModel {
            id: Set(id),
            ..Default::default()
        }
        .delete(&db)
        .await?;

        Ok(true)
    }
}

impl Query {
    pub async fn find_scan_batch_by_id(id: i32) -> anyhow::Result<Option<scan_batch::Model>> {
        let db = get_db_connection().await?;
        let model = scan_batch::Entity::find_by_id(id).one(&db).await?;

        Ok(model)
    }

    pub async fn find_last_scan_batch() -> anyhow::Result<Option<scan_batch::Model>> {
        let db = get_db_connection().await?;
        let model = scan_batch::Entity::find()
            .order_by_desc(scan_batch::Column::Id)
            .one(&db)
            .await?;

        Ok(model)
    }

    pub async fn find_open_scan_batch() -> anyhow::Result<Vec<scan_batch::Model>> {
        let db = get_db_connection().await?;
        let models = scan_batch::Entity::find()
            .filter(scan_batch::Column::End.is_null())
            .order_by_asc(scan_batch::Column::UpdatedAt)
            .all(&db)
            .await?;

        Ok(models)
    }

    pub async fn next_scan_batch() -> anyhow::Result<scan_batch::Model> {
        let scans = Self::find_open_scan_batch().await?;
        let date = chrono::Utc::now().with_timezone(&chrono::FixedOffset::east_opt(0).unwrap());

        #[allow(unused_assignments)]
        let mut scan: Option<scan_batch::Model> = None;
        if scans.len() == 0 {
            scan = Self::find_last_scan_batch().await?;
            if scan.is_none() {
                scan = Some(
                    Mutation::create_scan_batch(scan_batch::ActiveModel {
                        ip: Set("0.0.0.0".to_string()),
                        cursor: Set(0),
                        start: Set(date),
                        size: Set(BATCH_SIZE),
                        ..Default::default()
                    })
                    .await?,
                );
            } else {
                let new_cursor = scan.as_ref().unwrap().cursor + scan.as_ref().unwrap().size;
                scan = Some(
                    Mutation::create_scan_batch(scan_batch::ActiveModel {
                        ip: Set(convert_i32_to_ipv4_string(new_cursor)),
                        cursor: Set(new_cursor),
                        start: Set(date),
                        size: Set(BATCH_SIZE),
                        ..Default::default()
                    })
                    .await?,
                );
            }
        } else {
            scan = Some(scans[0].clone());
            Mutation::update_scan_batch(scan_batch::ActiveModel {
                id: Set(scan.as_ref().unwrap().id),
                updated_at: Set(Some(date)),
                ..Default::default()
            })
            .await?;
        }

        Ok(scan.unwrap())
    }
}
