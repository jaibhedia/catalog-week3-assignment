use deadpool_postgres::Pool;
use crate::models::{QueryParams, DepthPrice, Swap, Earnings, RunePool, PoolActivity};
use tokio_postgres::Row;

#[derive(Clone)]
pub struct Database {
    pub pool: Pool,
}

impl Database {
    pub fn new(pool: Pool) -> Self {
        Self { pool }
    }

    pub async fn find_depths(&self, params: &QueryParams) -> Result<Vec<DepthPrice>, Box<dyn std::error::Error>> {
        self.find_records("depth_price_history", params).await
    }

    pub async fn find_swaps(&self, params: &QueryParams) -> Result<Vec<Swap>, Box<dyn std::error::Error>> {
        self.find_records("swaps_history", params).await
    }

    pub async fn find_earnings(&self, params: &QueryParams) -> Result<Vec<Earnings>, Box<dyn std::error::Error>> {
        self.find_records("earnings_history", params).await
    }

    pub async fn find_runepools(&self, params: &QueryParams) -> Result<Vec<RunePool>, Box<dyn std::error::Error>> {
        self.find_records("rune_pool_history", params).await
    }

    pub async fn find_pool_activity(&self, pool_id: &str, params: &QueryParams) -> Result<Vec<PoolActivity>, Box<dyn std::error::Error>> {
        let client = self.pool.get().await?;
        let mut query = String::from(
            "SELECT d.pool, d.asset_depth, d.rune_depth, d.asset_price, 
                    COALESCE(s.to_asset_volume, 0) AS to_asset_volume, 
                    COALESCE(s.total_fees, 0) AS total_fees, 
                    COALESCE(s.total_volume_usd, 0) AS total_volume_usd, 
                    d.start_time, d.end_time
             FROM depth_price_history d
             LEFT JOIN swaps_history s ON d.pool = s.pool AND d.start_time = s.start_time AND d.end_time = s.end_time
             WHERE d.pool = $1"
        );
        let mut query_params: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = Vec::new();
        query_params.push(&pool_id);

        if let Some(date_range) = &params.date_range {
            query.push_str(" AND d.start_time >= $2 AND d.end_time <= $3");
            query_params.push(&date_range.0);
            query_params.push(&date_range.1);
        } else {
            if let Some(start_date) = &params.start_date {
                query.push_str(" AND d.start_time >= $2");
                query_params.push(start_date);
            }
            if let Some(end_date) = &params.end_date {
                let param_num = if query_params.len() == 1 { 2 } else { 3 };
                query.push_str(&format!(" AND d.end_time <= ${}", param_num));
                query_params.push(end_date);
            }
        }
        if let Some(liquidity_gt) = &params.liquidity_gt {
            let param_num = query_params.len() + 1;
            query.push_str(&format!(" AND d.asset_depth > ${}", param_num));
            query_params.push(liquidity_gt);
        }

        if let Some(ref sort_by) = params.sort_by {
            let order = params.order.as_deref().unwrap_or("asc");
            query.push_str(&format!(" ORDER BY {} {}", sort_by, order));
        } else {
            query.push_str(" ORDER BY d.start_time DESC");
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
        conditions.push(format!("start_time >= '{}'", start));
        conditions.push(format!("end_time <= '{}'", end));
    } else {
        if let Some(start_date) = params.start_date {
            conditions.push(format!("start_time >= '{}'", start_date));
        }
        if let Some(end_date) = params.end_date {
            conditions.push(format!("end_time <= '{}'", end_date));
        }
    }
    if let Some(liquidity_gt) = params.liquidity_gt {
        if table == "depth_price_history" {
            conditions.push(format!("asset_depth > {}", liquidity_gt));
        }
    }

    if !conditions.is_empty() {
        query.push_str(" WHERE ");
        query.push_str(&conditions.join(" AND "));
    }

    if let Some(ref sort_by) = params.sort_by {
        let order = params.order.as_deref().unwrap_or("asc");
        query.push_str(&format!(" ORDER BY {} {}", sort_by, order));
    } else {
        query.push_str(" ORDER BY start_time DESC");
    }

    let limit = params.limit.unwrap_or(10).min(100);
    let offset = params.page.unwrap_or(0) * limit;
    query.push_str(&format!(" LIMIT {} OFFSET {}", limit, offset));

    query
}