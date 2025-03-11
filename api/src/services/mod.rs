use deadpool_postgres::Pool;
use crate::db::Database;
use crate::models::{DepthPrice, Swap, Earnings, RunePool, PoolActivity, QueryParams};
use crate::fetcher::{fetch_depth_data, fetch_swaps_data, fetch_earnings_data, fetch_runepool_data};

#[derive(Clone)]
pub struct DepthService {
    db: Database,
}

impl DepthService {
    pub fn new(pool: Pool) -> Self {
        Self { db: Database::new(pool) }
    }

    pub async fn get_depths(&self, params: &QueryParams) -> Result<Vec<DepthPrice>, Box<dyn std::error::Error>> {
        self.db.find_depths(params).await
    }

    pub async fn get_swaps(&self, params: &QueryParams) -> Result<Vec<Swap>, Box<dyn std::error::Error>> {
        self.db.find_swaps(params).await
    }

    pub async fn get_earnings(&self, params: &QueryParams) -> Result<Vec<Earnings>, Box<dyn std::error::Error>> {
        self.db.find_earnings(params).await
    }

    pub async fn get_runepools(&self, params: &QueryParams) -> Result<Vec<RunePool>, Box<dyn std::error::Error>> {
        self.db.find_runepools(params).await
    }

    pub async fn get_pool_activity(&self, pool_id: String, params: &QueryParams) -> Result<Vec<PoolActivity>, Box<dyn std::error::Error>> {
        self.db.find_pool_activity(&pool_id, params).await
    }

    pub async fn fetch_and_store_depths(&self, client: &reqwest::Client) -> Result<(), Box<dyn std::error::Error>> {
        fetch_depth_data(&self.db.pool, client).await
    }

    pub async fn fetch_and_store_swaps(&self, client: &reqwest::Client) -> Result<(), Box<dyn std::error::Error>> {
        fetch_swaps_data(&self.db.pool, client).await
    }

    pub async fn fetch_and_store_earnings(&self, client: &reqwest::Client) -> Result<(), Box<dyn std::error::Error>> {
        fetch_earnings_data(&self.db.pool, client).await
    }

    pub async fn fetch_and_store_runepools(&self, client: &reqwest::Client) -> Result<(), Box<dyn std::error::Error>> {
        fetch_runepool_data(&self.db.pool, client).await
    }
}