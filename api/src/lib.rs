use async_graphql::{
    dataloader::DataLoader,
    http::{playground_source, GraphQLPlaygroundConfig},
};
use async_graphql_poem::GraphQL;
use dotenvy::dotenv;
use poem::{get, handler, listener::TcpListener, web::Html, IntoResponse, Route, Server};
use sea_orm::{Database, DatabaseConnection};
use std::sync::OnceLock;
pub mod query_root;

pub static URL: OnceLock<String> = OnceLock::new();
pub static ENDPOINT: OnceLock<String> = OnceLock::new();
pub static DATABASE_URL: OnceLock<String> = OnceLock::new();
pub static DEPTH_LIMIT: OnceLock<usize> = OnceLock::new();
pub static COMPLEXITY_LIMIT: OnceLock<usize> = OnceLock::new();

pub struct OrmDataloader {
    pub db: DatabaseConnection,
}

#[handler]
async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new(
        ENDPOINT.get().unwrap(),
    )))
}

#[tokio::main]
async fn start() -> anyhow::Result<()> {
    dotenv().ok();

    let database = Database::connect(DATABASE_URL.get().unwrap())
        .await
        .expect("Fail to initialize database connection");
    let orm_dataloader: DataLoader<OrmDataloader> = DataLoader::new(
        OrmDataloader {
            db: database.clone(),
        },
        tokio::spawn,
    );
    let schema = query_root::schema(
        database,
        orm_dataloader,
        DEPTH_LIMIT.get(),
        COMPLEXITY_LIMIT.get(),
    )
    .unwrap();
    let app = Route::new().at(
        ENDPOINT.get().unwrap(),
        get(graphql_playground).post(GraphQL::new(schema)),
    );
    println!("Visit GraphQL Playground at http://{}", URL.get().unwrap());
    Server::new(TcpListener::bind(URL.get().unwrap()))
        .run(app)
        .await
        .expect("Fail to start web server");

    Ok(())
}

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}
