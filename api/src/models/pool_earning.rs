use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

#[derive(Debug, Serialize, Deserialize)]
pub struct PoolEarning {
    pub id: i32,
    pub earnings_history_id: i32,
    pub pool: String,
    pub asset_liquidity_fees: i64,
    pub rune_liquidity_fees: i64,
    pub total_liquidity_fees_rune: i64,
    pub saver_earning: i64,
    pub rewards: i64,
    pub earnings: i64,
}

impl From<Row> for PoolEarning {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            earnings_history_id: row.get("earnings_history_id"),
            pool: row.get("pool"),
            asset_liquidity_fees: row.get("asset_liquidity_fees"),
            rune_liquidity_fees: row.get("rune_liquidity_fees"),
            total_liquidity_fees_rune: row.get("total_liquidity_fees_rune"),
            saver_earning: row.get("saver_earning"),
            rewards: row.get("rewards"),
            earnings: row.get("earnings"),
        }
    }
}