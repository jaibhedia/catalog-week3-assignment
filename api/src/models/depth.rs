// src/models/depth.rs
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use tokio_postgres::Row;

#[derive(Debug, Serialize, Deserialize)]
pub struct Depth {
    pub pool: String,  
    pub asset_depth: i64,
    pub rune_depth: i64,
    pub asset_price: f64,
    pub timestamp: DateTime<Utc>,
}

impl From<Row> for Depth {
    fn from(row: Row) -> Self {
        Self {
            pool: row.get("pool"),  // Changed from "pool_id" to "pool"
            asset_depth: row.get("asset_depth"),
            rune_depth: row.get("rune_depth"),
            asset_price: row.get("asset_price"),
            timestamp: row.get("timestamp"),
        }
    }
}
