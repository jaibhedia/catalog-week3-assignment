use api::jobs::setup_jobs;
use deadpool_postgres::{Config as PgConfig, Runtime};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut pg_config = PgConfig::new();
    pg_config.url = Some(database_url);
    let pool = pg_config.create_pool(Some(Runtime::Tokio1), tokio_postgres::NoTls)?;
    setup_jobs(pool).await?;
    Ok(())
}