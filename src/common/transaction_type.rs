use serde::Serialize;

#[derive(Serialize)]
pub enum TransactionType {
    Disbursement,
    Payment,
    RemittancePayout,
    Transfer,
    Refund
}
impl ToString for TransactionType {
    fn to_string(&self) -> String {
        match self {
            TransactionType::Disbursement => String::from("DISBURSEMENT"),
            TransactionType::Payment => String::from("PAYMENT"),
            TransactionType::RemittancePayout => String::from("REMITTANCE_PAYOUT"),
            TransactionType::Transfer => String::from("TRANSFER"),
            TransactionType::Refund => String::from("REFUND")
        }
    }
}