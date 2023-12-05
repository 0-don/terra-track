use crate::OrmDataloader;
use async_graphql::{dataloader::DataLoader, dynamic::*};
use entity::ip_main;
use sea_orm::DatabaseConnection;
use seaography::{Builder, BuilderContext};
use std::sync::OnceLock;

pub static CONTEXT: OnceLock<BuilderContext> = OnceLock::new();

pub fn schema(
    database: DatabaseConnection,
    orm_dataloader: DataLoader<OrmDataloader>,
    depth: Option<&usize>,
    complexity: Option<&usize>,
) -> Result<Schema, SchemaError> {
    let _ = CONTEXT.set(BuilderContext::default());
    let mut builder = Builder::new(&CONTEXT.get().unwrap());
    seaography::register_entities!(builder, [ip_main]);
    let schema = builder.schema_builder();
    let schema = if let Some(depth) = depth {
        schema.limit_depth(*depth)
    } else {
        schema
    };
    let schema = if let Some(complexity) = complexity {
        schema.limit_complexity(*complexity)
    } else {
        schema
    };
    schema.data(database).data(orm_dataloader).finish()
}
