use serde::Serialize;

#[derive(Serialize)]
pub enum TransactionStatus {
    Pending,
    Success,
    Failed,
    Voided,
    Reversed
}
impl ToString for TransactionStatus {
    fn to_string(&self) -> String {
        match self {
            TransactionStatus::Pending => String::from("PENDING"),
            TransactionStatus::Success => String::from("SUCCESS"),
            TransactionStatus::Failed => String::from("FAILED"),
            TransactionStatus::Voided => String::from("VOIDED"),
            TransactionStatus::Reversed => String::from("REVERSED")
        }
    }
}