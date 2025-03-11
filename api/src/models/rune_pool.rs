use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use tokio_postgres::Row;

#[derive(Debug, Serialize, Deserialize)]
pub struct RunePool {
    pub id: i32,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub count: i64,
    pub units: i64,
}

impl From<Row> for RunePool {
    fn from(row: Row) -> Self {
        Self {
            id: row.get("id"),
            start_time: row.get("start_time"),
            end_time: row.get("end_time"),
            count: row.get("count"),
            units: row.get("units"),
        }
    }
}