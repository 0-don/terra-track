pub mod query_root;
use crate::query_root::OrmDataloader;
use async_graphql::{
    dataloader::DataLoader,
    http::{playground_source, GraphQLPlaygroundConfig},
};
use async_graphql_poem::GraphQL;
use dotenvy::dotenv;
use migration::db::get_db_connection;
use poem::{
    get, handler, listener::TcpListener, middleware::Cors, web::Html, EndpointExt, IntoResponse,
    Route, Server,
};
use std::sync::OnceLock;

pub static URL: OnceLock<String> = OnceLock::new();
pub static ENDPOINT: OnceLock<String> = OnceLock::new();
pub static DATABASE_URL: OnceLock<String> = OnceLock::new();
pub static DEPTH_LIMIT: OnceLock<usize> = OnceLock::new();
pub static COMPLEXITY_LIMIT: OnceLock<usize> = OnceLock::new();

#[handler]
async fn graphql_playground() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new(
        ENDPOINT.get().unwrap(),
    )))
}

#[tokio::main]
async fn start() -> anyhow::Result<()> {
    dotenv().ok();
    setup();
    let database = get_db_connection().await?;
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

    let cors = Cors::new()
        .allow_origin("http://localhost:4000")
        .allow_origin("https://studio.apollographql.com")
        .allow_methods(["POST", "GET", "OPTIONS"])
        .allow_credentials(true);

    let app = Route::new().at(
        ENDPOINT.get().unwrap(),
        get(graphql_playground)
            .post(GraphQL::new(schema))
            .with(cors),
    );
    println!("Visit GraphQL Playground at http://{}", URL.get().unwrap());
    Server::new(TcpListener::bind(URL.get().unwrap()))
        .run(app)
        .await
        .expect("Fail to start web server");

    Ok(())
}

fn setup() {
    dotenv().ok();
    let _ = URL.set(std::env::var("URL").unwrap_or("0.0.0.0:4000".into()));
    let _ = ENDPOINT.set(std::env::var("ENDPOINT").unwrap_or("/graphql".into()));
    let _ = DATABASE_URL.set(std::env::var("DATABASE_URL").unwrap_or("".into()));
    let _ = DEPTH_LIMIT.set(
        std::env::var("DEPTH_LIMIT")
            .unwrap_or("10".into())
            .parse::<usize>()
            .unwrap(),
    );
    let _ = COMPLEXITY_LIMIT.set(
        std::env::var("COMPLEXITY_LIMIT")
            .unwrap_or("1000".into())
            .parse::<usize>()
            .unwrap(),
    );
}

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}
