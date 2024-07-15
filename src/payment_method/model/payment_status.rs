use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PaymentStatus {
    Active,
    Expired,
    Inactive,
    Pending,
    RequiresAction,
    Failed,
    Succeeded,
    XenditEnumDefaultFallback,
}