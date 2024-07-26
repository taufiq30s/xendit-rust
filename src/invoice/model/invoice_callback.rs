use chrono::{DateTime, Utc};
use serde::Deserialize;

use super::{InvoiceFee, InvoiceItem, PaymentDetails};

#[derive(Deserialize)]
pub struct InvoiceCallback {
    pub id: String,
    pub external_id: String,
    pub user_id: String,
    pub is_high: bool,
    pub status: String,
    pub merchant_name: String,
    pub amount: f64,
    pub payer_email: Option<String>,
    pub description: Option<String>,
    pub paid_amount: Option<f64>,
    pub updated: DateTime<Utc>,
    pub created: DateTime<Utc>,
    pub currency: String,
    pub paid_at: Option<String>,
    pub payment_method: Option<String>,
    pub payment_channel: Option<String>,
    pub payment_destination: Option<String>,
    pub payment_details: Option<PaymentDetails>,
    pub payment_id: String,
    pub success_redirect_url: String,
    pub failure_redirect_url: String,
    pub credit_card_charge_id: Option<String>,
    pub items: Option<Vec<InvoiceItem>>,
    pub fees: Option<Vec<InvoiceFee>>,
    pub should_authenticate_credit_card: Option<bool>,
    pub bank_code: Option<String>,
    pub ewallet_type: Option<String>,
    pub on_demand_link: Option<String>,
    pub receipt_id: Option<String>,
}