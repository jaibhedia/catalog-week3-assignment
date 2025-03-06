use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use tokio_postgres::Row;

#[derive(Debug, Serialize, Deserialize)]
pub struct Swap {
    pub pool: String,
    pub from_asset: String,
    pub to_asset: String,
    pub amount: i64,
    pub fee: i64,
    pub volume_usd: f64,
    pub timestamp: DateTime<Utc>,
}

impl From<Row> for Swap {
    fn from(row: Row) -> Self {
        Self {
            pool: row.get("pool"),
            from_asset: row.get("from_asset"),
            to_asset: row.get("to_asset"),
            amount: row.get("amount"),
            fee: row.get("fee"),
            volume_usd: row.get("volume_usd"),
            timestamp: row.get("timestamp"),
        }
    }
}