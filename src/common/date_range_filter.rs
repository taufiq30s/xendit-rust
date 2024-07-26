use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct DateRangeFilter {
    gt: Option<DateTime<Utc>>,
    lt: Option<DateTime<Utc>>,
}