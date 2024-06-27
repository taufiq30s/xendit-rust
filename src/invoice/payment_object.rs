use serde::Deserialize;

#[derive(Deserialize)]
pub struct Bank {
    pub bank_code: String,
    pub collection_type: String,
    pub transfer_amount: f64,           // Will be deprected soon
    pub bank_branch: String,
    pub account_holder_name: String,
}

#[derive(Deserialize)]
pub struct RetailOutlet {
    pub retail_outlet_name: String,
    pub transfer_amount: f64        // Will be depracted
}

#[derive(Deserialize)]
pub struct Ewallet {
    pub ewallet_type: String
}

#[derive(Deserialize)]
pub struct QRCode {
    pub qr_code_type: String
}

#[derive(Deserialize)]
pub struct DirectDebit {
    pub direct_debit_type: String
}

#[derive(Deserialize)]
pub struct Paylater {
    pub paylater_type: String
}

#[derive(Deserialize)]
pub struct PaymentDetail {
    pub receipt_id: Option<String>,
    pub source: Option<String>
}