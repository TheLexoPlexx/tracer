#![allow(dead_code)]

use dotenv::dotenv;
use poem::{EndpointExt, Route, Server, listener::TcpListener};
use poem_openapi::OpenApiService;
use sqlx::{PgPool, Pool, Postgres};
use std::{
    env::var,
    fs::{self},
};
use tokio::sync::mpsc::Sender;
use tokio::time::Duration;
use tracerapi::TracerApi;

#[derive(Clone)]
struct ApiData(Option<PgPool>, Sender<String>);

impl ApiData {
    fn postgres_pool(&self) -> &Option<PgPool> {
        &self.0
    }

    fn websocket_channel(&self) -> &Sender<String> {
        &self.1
    }
}

mod structure;
mod tracerapi;
mod util;
mod websocket_handler;

#[tokio::main]
async fn main() {
    println!("Starting server...");

    dotenv().ok();

    if std::env::var_os("RUST_LOG").is_none() {
        unsafe {
            std::env::set_var("RUST_LOG", "poem=debug");
        }
    }
    tracing_subscriber::fmt::init();

    let host = var("HOST").unwrap_or("0.0.0.0".to_string());
    let port = var("PORT").unwrap_or("4000".to_string());
    let version = var("CARGO_PKG_VERSION").expect("Failed to read Cargo.toml");
    let pg_pool: Option<Pool<Postgres>> = match var("DATABASE_URL") {
        Ok(pg_url) => match PgPool::connect(&pg_url).await {
            Ok(pool) => Some(pool),
            Err(e) => {
                eprintln!("> Not connected to PostgreSQL-Database: {}", e);
                return;
            }
        },
        Err(_) => None,
    };

    let api_path = "/";
    let explorer_path = "/explorer";
    let endpoint_path = "/endpoint";

    let description = match fs::read_to_string("description.html") {
        Ok(desc) => desc,
        Err(_) => {
            println!("Failed to read description.html. Using default.");
            "<b>Tracer-API</b></br><p>Failed to read description.html</p>".to_string()
        }
    };

    let api_service = OpenApiService::new(TracerApi, "Tracer-API", "0.1.0")
        .server(api_path)
        .description(description);
    let explorer = api_service.openapi_explorer();

    let websocket_channel = tokio::sync::mpsc::channel::<String>(32);

    let data = if pg_pool.is_some() {
        println!("> Connected to PostgreSQL-Database");
        ApiData(pg_pool, websocket_channel.0)
    } else {
        ApiData(None, websocket_channel.0)
    };

    let route = Route::new()
        .nest(
            endpoint_path.to_string() + "/yaml",
            api_service.spec_endpoint_yaml(),
        )
        .nest(
            endpoint_path.to_string() + "/json",
            api_service.spec_endpoint(),
        )
        .at("/ws/:name", websocket_handler::websocket)
        .nest(api_path, api_service)
        .nest(explorer_path, explorer)
        .data(data);

    let listener_adress = format!("{host}:{port}");

    println!("> Explorer: http://{listener_adress}{explorer_path}");
    println!("> API: http://{listener_adress}{api_path}");
    println!("> Version: {version}");

    //  TODO: Refactor, so that the server is only created once

    Server::new(TcpListener::bind(&listener_adress))
        .name("Tracer-API")
        .run_with_graceful_shutdown(
            route,
            async move {
                let _ = tokio::signal::ctrl_c().await;
                println!("Received Ctrl+C, Shutting down...");
            },
            Some(Duration::from_secs(5)), //TODO: Was macht das?
        )
        .await
        .expect("Failed to start server.");
}
