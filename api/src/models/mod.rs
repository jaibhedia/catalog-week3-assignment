pub mod depth;
pub mod swap;
pub mod earning;
pub mod runepool;

pub use depth::Depth;
pub use swap::Swap;
pub use earning::Earning;
pub use runepool::RunePool;

use serde::{Serialize, Deserialize, Deserializer};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct PoolActivity {
    pub pool: String,
    pub asset_depth: i64,
    pub rune_depth: i64,
    pub asset_price: f64,
    pub swap_amount: i64,
    pub swap_fee: i64,
    pub volume_usd: f64,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryParams {
    #[serde(default, deserialize_with = "deserialize_date_range")]
    pub date_range: Option<(DateTime<Utc>, DateTime<Utc>)>, // New field
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub liquidity_gt: Option<i64>,
    pub sort_by: Option<String>,
    pub order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

// Custom deserializer for date_range
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