pub mod depth;
pub mod swap;
pub mod earning;
pub mod runepool;

pub use depth::Depth;
pub use swap::Swap;
pub use earning::Earning;
pub use runepool::RunePool;

// Add PoolActivity
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
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub liquidity_gt: Option<i64>,
    pub sort_by: Option<String>,
    pub order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
}