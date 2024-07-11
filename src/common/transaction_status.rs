use serde::Serialize;

#[derive(Serialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionStatus {
    Pending,
    Success,
    Failed,
    Voided,
    Reversed
}