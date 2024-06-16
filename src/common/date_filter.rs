use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct DateFilter {
    gt: Option<DateTime<Utc>>,
    lt: Option<DateTime<Utc>>,
}