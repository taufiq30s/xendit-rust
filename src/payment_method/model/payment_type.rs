use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PaymentType {
    Card,
    Cryptocurrency,
    DirectBankTransfer,
    DirectDebit,
    Ewallet,
    OverTheCounter,
    QrCode,
    VirtualAccount,
    XenditEnumDefaultFallback
}