use crate::fetcher::{fetch_depth_data, fetch_swaps_data};
use deadpool_postgres::Pool;
use tokio_cron_scheduler::{JobScheduler, Job};

pub async fn setup_jobs(pool: Pool) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let sched = JobScheduler::new().await?;
    let client = reqwest::Client::new();

    let job = Job::new_async("0 0 * * * *", move |_, _| {
        let client = client.clone();
        let pool = pool.clone();
        Box::pin(async move {
            fetch_depth_data(&pool, &client).await.unwrap_or_else(|e| eprintln!("Depth error: {}", e));
            fetch_swaps_data(&pool, &client).await.unwrap_or_else(|e| eprintln!("Swaps error: {}", e));
        })
    })?;

    sched.add(job).await?;
    sched.start().await?;
    Ok(())
}