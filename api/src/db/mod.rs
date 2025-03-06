use deadpool_postgres::Pool;
use crate::models::{QueryParams, Depth, Swap, Earning, RunePool};

#[derive(Clone)]
pub struct Database {
    pub pool: Pool,
}

impl Database {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    pub async fn find_depths(&self, params: &QueryParams) -> Result<Vec<Depth>, Box<dyn std::error::Error>> {
        self.find_records("depth_history", params).await
    }

    pub async fn find_swaps(&self, params: &QueryParams) -> Result<Vec<Swap>, Box<dyn std::error::Error>> {
        self.find_records("swaps_history", params).await
    }

    pub async fn find_earnings(&self, params: &QueryParams) -> Result<Vec<Earning>, Box<dyn std::error::Error>> {
        self.find_records("earnings_history", params).await
    }

    pub async fn find_runepools(&self, params: &QueryParams) -> Result<Vec<RunePool>, Box<dyn std::error::Error>> {
        self.find_records("runepool_history", params).await
    }

    async fn find_records<T: From<tokio_postgres::Row> + Send + Sync>(&self, table: &str, params: &QueryParams) -> Result<Vec<T>, Box<dyn std::error::Error>> {
        let client = self.pool.get().await?;
        let query = build_query(table, params);
        let rows = client.query(&query, &[]).await?;
        Ok(rows.into_iter().map(T::from).collect())
    }
}

fn build_query(table: &str, params: &QueryParams) -> String {
    let mut query = format!("SELECT * FROM {}", table);
    let mut conditions = Vec::new();

    if let Some(start_date) = params.start_date {
        conditions.push(format!("timestamp >= '{}'", start_date));
    }
    if let Some(end_date) = params.end_date {
        conditions.push(format!("timestamp <= '{}'", end_date));
    }
    if let Some(liquidity_gt) = params.liquidity_gt {
        conditions.push(format!("asset_depth > {}", liquidity_gt)); // Adjust for each table if needed
    }

    if !conditions.is_empty() {
        query.push_str(" WHERE ");
        query.push_str(&conditions.join(" AND "));
    }

    if let Some(ref sort_by) = params.sort_by {
        let order = params.order.as_deref().unwrap_or("asc");
        query.push_str(&format!(" ORDER BY {} {}", sort_by, order));
    }

    let limit = params.limit.unwrap_or(10).min(100);
    let offset = params.page.unwrap_or(0) * limit;
    query.push_str(&format!(" LIMIT {} OFFSET {}", limit, offset));

    query
}