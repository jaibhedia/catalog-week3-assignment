pub mod depth_price;
pub mod swap;
pub mod earnings;
pub mod pool_earning;
pub mod rune_pool;

pub use depth_price::DepthPrice;
pub use swap::Swap;
pub use earnings::Earnings;
pub use pool_earning::PoolEarning;
pub use rune_pool::RunePool;

use serde::{Serialize, Deserialize, Deserializer};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct PoolActivity {
    pub pool: String,
    pub asset_depth: i64,
    pub rune_depth: i64,
    pub asset_price: f64,
    pub to_asset_volume: i64,
    pub total_fees: i64,
    pub total_volume_usd: i64,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
}

impl From<tokio_postgres::Row> for PoolActivity {
    fn from(row: tokio_postgres::Row) -> Self {
        Self {
            pool: row.get("pool"),
            asset_depth: row.get("asset_depth"),
            rune_depth: row.get("rune_depth"),
            asset_price: row.get("asset_price"),
            to_asset_volume: row.get("to_asset_volume"),
            total_fees: row.get("total_fees"),
            total_volume_usd: row.get("total_volume_usd"),
            start_time: row.get("start_time"),
            end_time: row.get("end_time"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryParams {
    #[serde(default, deserialize_with = "deserialize_date_range")]
    pub date_range: Option<(DateTime<Utc>, DateTime<Utc>)>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub liquidity_gt: Option<i64>,
    pub sort_by: Option<String>,
    pub order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

fn deserialize_date_range<'de, D>(deserializer: D) -> Result<Option<(DateTime<Utc>, DateTime<Utc>)>, D::Error>
where D: Deserializer<'de> {
    let s: Option<String> = Option::deserialize(deserializer)?;
    s.map(|s| {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 2 {
            return Err(serde::de::Error::custom("date_range must be in format 'start,end'"));
        }
        let start = DateTime::parse_from_rfc3339(parts[0])
            .map_err(serde::de::Error::custom)?
            .with_timezone(&Utc);
        let end = DateTime::parse_from_rfc3339(parts[1])
            .map_err(serde::de::Error::custom)?
            .with_timezone(&Utc);
        Ok((start, end))
    }).transpose()
}