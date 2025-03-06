use crate::services::DepthService;
use deadpool_postgres::Pool;
use tokio_cron_scheduler::{JobScheduler, Job};

pub async fn setup_jobs(pool: Pool) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let sched = JobScheduler::new().await?;
    let client = reqwest::Client::new();
    let service = DepthService::new(pool.clone());

    let job = Job::new_async("0 0 * * * *", move |_, _| { // Hourly schedule
        let client = client.clone();
        let service = service.clone();
        Box::pin(async move {
            service.fetch_and_store_depths(&client).await.unwrap_or_else(|e| eprintln!("Depth error: {}", e));
            service.fetch_and_store_swaps(&client).await.unwrap_or_else(|e| eprintln!("Swaps error: {}", e));
            service.fetch_and_store_earnings(&client).await.unwrap_or_else(|e| eprintln!("Earnings error: {}", e));
            service.fetch_and_store_runepools(&client).await.unwrap_or_else(|e| eprintln!("Runepool error: {}", e));
        })
    })?;

    sched.add(job).await?;
    sched.start().await?;
    Ok(())
}