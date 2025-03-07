use deadpool_postgres::Pool;
use crate::models::{QueryParams, Depth, Swap, Earning, RunePool, PoolActivity};
use tokio_postgres::Row;

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

    pub async fn find_pool_activity(&self, pool_id: &str, params: &QueryParams) -> Result<Vec<PoolActivity>, Box<dyn std::error::Error>> {
        let client = self.pool.get().await?;
        let mut conditions = Vec::new();
        let mut query_params: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = Vec::new();

        let mut query = String::from(
            "SELECT d.pool, d.asset_depth, d.rune_depth, d.asset_price, 
                    COALESCE(s.amount, 0) AS swap_amount, COALESCE(s.fee, 0) AS swap_fee, 
                    COALESCE(s.volume_usd, 0.0) AS volume_usd, d.timestamp
             FROM depth_history d
             LEFT JOIN swaps_history s ON d.pool = s.pool AND d.timestamp = s.timestamp
             WHERE d.pool = $1"
        );
        query_params.push(&pool_id);

        // Borrow from params.date_range directly
        if let Some(date_range) = &params.date_range {
            conditions.push(format!("d.timestamp >= ${}", query_params.len() + 1));
            query_params.push(&date_range.0); // Reference to first element of tuple
            conditions.push(format!("d.timestamp <= ${}", query_params.len() + 1));
            query_params.push(&date_range.1); // Reference to second element of tuple
        } else {
            if let Some(start_date) = &params.start_date {
                conditions.push(format!("d.timestamp >= ${}", query_params.len() + 1));
                query_params.push(start_date); // Direct reference to start_date
            }
            if let Some(end_date) = &params.end_date {
                conditions.push(format!("d.timestamp <= ${}", query_params.len() + 1));
                query_params.push(end_date); // Direct reference to end_date
            }
        }
        if let Some(liquidity_gt) = &params.liquidity_gt {
            conditions.push(format!("d.asset_depth > ${}", query_params.len() + 1));
            query_params.push(liquidity_gt); // Direct reference to liquidity_gt
        }

        if !conditions.is_empty() {
            query.push_str(" AND ");
            query.push_str(&conditions.join(" AND "));
        }

        if let Some(ref sort_by) = params.sort_by {
            let order = params.order.as_deref().unwrap_or("asc");
            query.push_str(&format!(" ORDER BY {} {}", sort_by, order));
        }

        let limit = params.limit.unwrap_or(10).min(100);
        let offset = params.page.unwrap_or(0) * limit;
        query.push_str(&format!(" LIMIT ${} OFFSET ${}", query_params.len() + 1, query_params.len() + 2));
        query_params.push(&limit);
        query_params.push(&offset);

        let rows = client.query(&query, &query_params[..]).await?;
        Ok(rows.into_iter().map(PoolActivity::from).collect())
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

    if let Some((start, end)) = params.date_range {
        conditions.push(format!("timestamp >= '{}'", start));
        conditions.push(format!("timestamp <= '{}'", end));
    } else {
        if let Some(start_date) = params.start_date {
            conditions.push(format!("timestamp >= '{}'", start_date));
        }
        if let Some(end_date) = params.end_date {
            conditions.push(format!("timestamp <= '{}'", end_date));
        }
    }
    if let Some(liquidity_gt) = params.liquidity_gt {
        conditions.push(format!("asset_depth > {}", liquidity_gt));
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

impl From<Row> for PoolActivity {
    fn from(row: Row) -> Self {
        Self {
            pool: row.get("pool"),
            asset_depth: row.get("asset_depth"),
            rune_depth: row.get("rune_depth"),
            asset_price: row.get("asset_price"),
            swap_amount: row.get("swap_amount"),
            swap_fee: row.get("swap_fee"),
            volume_usd: row.get("volume_usd"),
            timestamp: row.get("timestamp"),
        }
    }
}