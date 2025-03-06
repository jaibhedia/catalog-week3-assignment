pub mod depth;
pub mod swap;
pub mod earning;
pub mod runepool;

pub use depth::Depth;
pub use swap::Swap;
pub use earning::Earning;
pub use runepool::RunePool;

use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryParams {
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub liquidity_gt: Option<i64>,
    pub sort_by: Option<String>,
    pub order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
}