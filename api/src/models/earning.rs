use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use tokio_postgres::Row;

#[derive(Debug, Serialize, Deserialize)]
pub struct Earning {
    pub pool: String,
    pub liquidity_fees: i64,
    pub block_rewards: i64,
    pub timestamp: DateTime<Utc>,
}

impl From<Row> for Earning {
    fn from(row: Row) -> Self {
        Self {
            pool: row.get("pool"),
            liquidity_fees: row.get("liquidity_fees"),
            block_rewards: row.get("block_rewards"),
            timestamp: row.get("timestamp"),
        }
    }
}