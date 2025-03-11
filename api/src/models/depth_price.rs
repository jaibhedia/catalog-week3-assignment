use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use tokio_postgres::Row;

#[derive(Debug, Serialize, Deserialize)]
pub struct DepthPrice {
    pub id: i32,
    pub pool: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub asset_depth: i64,
    pub rune_depth: i64,
    pub asset_price: f64,
    pub asset_price_usd: f64,
    pub liquidity_units: i64,
    pub members_count: i64,
    pub synth_units: i64,
    pub synth_supply: i64,
    pub units: i64,
    pub luvi: f64,
}

impl From<Row> for DepthPrice {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            pool: row.get("pool"),
            start_time: row.get("start_time"),
            end_time: row.get("end_time"),
            asset_depth: row.get("asset_depth"),
            rune_depth: row.get("rune_depth"),
            asset_price: row.get("asset_price"),
            asset_price_usd: row.get("asset_price_usd"),
            liquidity_units: row.get("liquidity_units"),
            members_count: row.get("members_count"),
            synth_units: row.get("synth_units"),
            synth_supply: row.get("synth_supply"),
            units: row.get("units"),
            luvi: row.get("luvi"),
        }
    }
}