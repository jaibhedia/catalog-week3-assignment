use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use tokio_postgres::Row;

#[derive(Debug, Serialize, Deserialize)]
pub struct Earnings {
    pub id: i32,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub liquidity_fees: i64,
    pub block_rewards: i64,
    pub earnings: i64,
    pub bonding_earnings: i64,
    pub liquidity_earnings: i64,
    pub avg_node_count: f64,
    pub rune_price_usd: f64,
}

impl From<Row> for Earnings {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            start_time: row.get("start_time"),
            end_time: row.get("end_time"),
            liquidity_fees: row.get("liquidity_fees"),
            block_rewards: row.get("block_rewards"),
            earnings: row.get("earnings"),
            bonding_earnings: row.get("bonding_earnings"),
            liquidity_earnings: row.get("liquidity_earnings"),
            avg_node_count: row.get("avg_node_count"),
            rune_price_usd: row.get("rune_price_usd"),
        }
    }
}