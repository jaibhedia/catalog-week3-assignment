use deadpool_postgres::Pool;
use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize)]
pub struct PoolActivity {
    pool: String,
    asset_depth: i64,
    rune_depth: i64,
    asset_price: f64,
    swap_amount: i64,
    swap_fee: i64,
    volume_usd: f64,
    timestamp: DateTime<Utc>,
}

pub async fn get_pool_activity(
    pool: &Pool,
    pool_id: &str,
    start_date: Option<DateTime<Utc>>,
    end_date: Option<DateTime<Utc>>,
    limit: i64,
    offset: i64,
) -> Result<Vec<PoolActivity>, tokio_postgres::Error> {
    let client = pool.get().await?;
    let query = "
        SELECT 
            d.pool, 
            d.asset_depth, 
            d.rune_depth, 
            d.asset_price, 
            COALESCE(s.amount, 0) as swap_amount, 
            COALESCE(s.fee, 0) as swap_fee, 
            COALESCE(s.volume_usd, 0.0) as volume_usd, 
            d.timestamp
        FROM depth_history d
        LEFT JOIN swaps_history s 
            ON d.pool = s.pool AND d.timestamp = s.timestamp
        WHERE d.pool = $1
            AND ($2::timestamp IS NULL OR d.timestamp >= $2)
            AND ($3::timestamp IS NULL OR d.timestamp <= $3)
        ORDER BY d.timestamp DESC
        LIMIT $4 OFFSET $5
    ";
    let rows = client
        .query(query, &[&pool_id, &start_date, &end_date, &limit, &offset])
        .await?;
    Ok(rows.into_iter().map(|row| PoolActivity {
        pool: row.get("pool"),
        asset_depth: row.get("asset_depth"),
        rune_depth: row.get("rune_depth"),
        asset_price: row.get("asset_price"),
        swap_amount: row.get("swap_amount"),
        swap_fee: row.get("swap_fee"),
        volume_usd: row.get("volume_usd"),
        timestamp: row.get("timestamp"),
    }).collect())
}