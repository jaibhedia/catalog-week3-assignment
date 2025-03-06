use crate::services::DepthService;
use deadpool_postgres::Pool;
use tokio_cron_scheduler::{JobScheduler, Job};
use log::{info, error};

pub async fn setup_jobs(pool: Pool) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let sched = JobScheduler::new().await?;
    let client = reqwest::Client::new();
    let service = DepthService::new(pool.clone());

    info!("Setting up scheduled job");
    let job = Job::new_async("0 * * * * *", move |_, _| { // Every minute for testing
        let client = client.clone();
        let service = service.clone();
        Box::pin(async move {
            info!("Scheduled job running");
            if let Err(e) = service.fetch_and_store_depths(&client).await {
                error!("Depth error: {}", e);
            } else {
                info!("Depth data fetched and stored");
            }
            if let Err(e) = service.fetch_and_store_swaps(&client).await {
                error!("Swaps error: {}", e);
            } else {
                info!("Swaps data fetched and stored");
            }
            if let Err(e) = service.fetch_and_store_earnings(&client).await {
                error!("Earnings error: {}", e);
            } else {
                info!("Earnings data fetched and stored");
            }
            if let Err(e) = service.fetch_and_store_runepools(&client).await {
                error!("Runepool error: {}", e);
            } else {
                info!("Runepool data fetched and stored");
            }
        })
    })?;

    sched.add(job).await?;
    sched.start().await?;
    info!("Job scheduler started");
    Ok(())
}