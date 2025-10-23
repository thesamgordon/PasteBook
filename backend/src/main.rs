mod models;
mod utils;
mod database;
mod routes;
mod scheduler;

use crate::routes::configure_routes;
use database::postgres_service::PostgresService;
use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use std::env;
use tracing::info;
use tracing_subscriber::EnvFilter;
use crate::scheduler::scheduler::Scheduler;
use crate::scheduler::tasks::delete_handler::DeleteHandler;

#[cfg(feature="local-dev")]
fn load_env() {
    dotenv::dotenv().ok();
}

pub struct State {
    postgres_service: PostgresService,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_ansi(true)
        .init();

    info!("PasteBook backend service starting...");

    #[cfg(feature="local-dev")]
    load_env();

    let postgres_uri = env::var("POSTGRES_URI").expect("POSTGRES_URI must be set");
    let max_payload_size = env::var("MAX_PAYLOAD_SIZE").unwrap_or("10".to_string()).parse::<usize>().unwrap();

    let data = web::Data::new(State {
        postgres_service: PostgresService::new(&postgres_uri).await.unwrap(),
    });

    info!("Pre-bind complete. Starting server.");

    Scheduler::new(data.clone())
        .add_job(DeleteHandler {})
        .run();

    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .app_data(web::PayloadConfig::default().limit(max_payload_size * 1024 * 1024))
            .app_data(data.clone())
            .configure(configure_routes)
    })
        .bind(("::", 8080))?
        .run()
        .await
}