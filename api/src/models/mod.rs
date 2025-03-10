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

#[derive(Debug, Deserialize)]
pub struct QueryParams {
    pub date_range: Option<String>,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub liquidity_gt: Option<i64>,
    pub sort_by: Option<String>,
    pub order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

pub fn parse_date_range(date_range: &str) -> Result<(DateTime<Utc>, DateTime<Utc>), Box<dyn Error>> {
    let dates: Vec<&str> = date_range.split(',').collect();
    if dates.len() != 2 {
        return Err("Invalid date_range format. Expected 'start,end'".into());
    }
    let start = chrono::NaiveDate::parse_from_str(dates[0], "%Y-%m-%d")
        .map_err(|e| format!("Failed to parse start date: {}", e))?
        .and_hms_opt(0, 0, 0)
        .ok_or("Invalid start time")?
        .and_utc();
    let end = chrono::NaiveDate::parse_from_str(dates[1], "%Y-%m-%d")
        .map_err(|e| format!("Failed to parse end date: {}", e))?
        .and_hms_opt(23, 59, 59)
        .ok_or("Invalid end time")?
        .and_utc();
    Ok((start, end))
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