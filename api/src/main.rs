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
    dotenv().ok(); // Load .env for local development, ignored on Render if not present
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    // Use DATABASE_URL from Render environment, fallback for local testing
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:Bhakwaas@csd37@localhost:5432/api".to_string());
    let mut pg_config = PgConfig::new();
    pg_config.url = Some(database_url);

    let pool = pg_config.create_pool(Some(Runtime::Tokio1), tokio_postgres::NoTls)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

    let service = DepthService::new(pool.clone());
    setup_jobs(pool.clone()).await.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    // Use PORT from Render environment, default to 8080 for local testing
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let bind_address = format!("0.0.0.0:{}", port); // Bind to 0.0.0.0 for Render

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(service.clone()))
            .configure(config)
    })
    .bind(&bind_address)?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test;

    #[actix_rt::test]
    async fn test_all_endpoints() {
        let mut pg_config = PgConfig::new();
        pg_config.dbname = Some("api".to_string());
        pg_config.host = Some("localhost".to_string());
        pg_config.user = Some("postgres".to_string());
        pg_config.password = Some("Bhakwaas@csd37".to_string());

        let pool = pg_config.create_pool(Some(Runtime::Tokio1), tokio_postgres::NoTls).unwrap();
        let service = DepthService::new(pool);
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(service))
                .configure(config)
        ).await;

        for endpoint in ["/api/depth-history", "/api/swaps-history", "/api/earnings-history", "/api/runepool-history"] {
            let req = test::TestRequest::get()
                .uri(&format!("{}?limit=5", endpoint))
                .to_request();
            let resp = test::call_service(&app, req).await;
            assert!(resp.status().is_success(), "Failed for {}", endpoint);
        }
    }
}