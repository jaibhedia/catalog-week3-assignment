use actix_web::{App, HttpServer, web};
use deadpool_postgres::{Config as PgConfig, Runtime};
use dotenv::dotenv;
use std::env;
use crate::routes::config;
use crate::services::DepthService;
use crate::jobs::setup_jobs;

mod db;
mod fetcher;
mod jobs;
mod models;
mod routes;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:Bhakwaas@csd37@localhost:5432/api".to_string());
    let mut pg_config = PgConfig::new();
    pg_config.url = Some(database_url);

    let pool = pg_config.create_pool(Some(Runtime::Tokio1), tokio_postgres::NoTls)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

    let service = DepthService::new(pool.clone());
    setup_jobs(pool.clone()).await.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    let port = env::var("PORT").unwrap_or("8080".to_string());
    let bind_address = format!("0.0.0.0:{}", port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(service.clone()))
            .configure(config)
    })
    .bind(bind_address)?
    .run()
    .await
}