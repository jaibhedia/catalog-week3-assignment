use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use tokio_postgres::Row;

#[derive(Debug, Serialize, Deserialize)]
pub struct RunePool {
    pub total_units: i64,
    pub members_count: i32,
    pub value: i64,
    pub timestamp: DateTime<Utc>,
}

impl From<Row> for RunePool {
    fn from(row: Row) -> Self {
        Self {
            total_units: row.get("total_units"),
            members_count: row.get("members_count"),
            value: row.get("value"),
            timestamp: row.get("timestamp"),
        }
    }
}