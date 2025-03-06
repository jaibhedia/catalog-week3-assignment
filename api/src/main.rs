use actix_web::{App, HttpServer, web};
use deadpool_postgres::{Config as PgConfig, Runtime};
use dotenv::dotenv;
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

    let mut pg_config = PgConfig::new();
    pg_config.dbname = Some(std::env::var("DATABASE_NAME").unwrap_or("api".to_string()));
    pg_config.host = Some(std::env::var("DATABASE_HOST").unwrap_or("localhost".to_string()));
    pg_config.user = Some(std::env::var("DATABASE_USER").unwrap_or("postgres".to_string()));
    pg_config.password = Some(std::env::var("DATABASE_PASSWORD").unwrap_or("".to_string()));

    let pool = pg_config.create_pool(Some(Runtime::Tokio1), tokio_postgres::NoTls)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

    let service = DepthService::new(pool.clone());
    setup_jobs(pool.clone()).await.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(service.clone()))
            .configure(config)
    })
    .bind("127.0.0.1:8080")?
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