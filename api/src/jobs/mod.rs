use crate::fetcher::{fetch_depth_data, fetch_swaps_data};
use crate::services::DepthService;
use deadpool_postgres::Pool;
use tokio_cron_scheduler::{JobScheduler, Job};

pub async fn setup_jobs(pool: Pool) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let sched = JobScheduler::new().await?;
    let client = reqwest::Client::new();
    let service = DepthService::new(pool.clone());

    let job = Job::new_async("0 * * * * *", move |_, _| { // Every minute for testing
        let client = client.clone();
        let service = service.clone();
        Box::pin(async move {
            service.fetch_and_store_depths(&client).await.unwrap_or_else(|e| eprintln!("Depth error: {}", e));
            service.fetch_and_store_swaps(&client).await.unwrap_or_else(|e| eprintln!("Swaps error: {}", e));
            // Add earnings and runepool when implemented
        })
    })?;

    sched.add(job).await?;
    sched.start().await?;
    Ok(())
}