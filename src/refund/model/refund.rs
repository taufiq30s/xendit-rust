use std::collections::HashMap;

use serde::Deserialize;

use crate::common::Currency;

#[derive(Deserialize)]
pub struct Refund {
    id: String,
    payment_request_id: String,
    amount: f64,
    channel_code: String,
    country: String,
    currency: Currency,
    reference_id: Option<String>,
    failure_code: Option<String>,
    refund_fee_amount: Option<f64>,
    created: Option<String>,
    updated: Option<String>,
    metadata: Option<HashMap<String, serde_json::Value>>
}
impl Refund {
    pub fn get_id(&self) -> &str {
        &self.id
    }
    pub fn get_payment_request_id(&self) -> &str {
        &self.payment_request_id
    }
    pub fn get_amount(&self) -> f64 {
        self.amount
    }
    pub fn get_channel_code(&self) -> &str {
        &self.channel_code
    }
    pub fn get_country(&self) -> &str {
        &self.country
    }
    pub fn get_currency(&self) -> &Currency {
        &self.currency
    }
    pub fn get_reference_id(&self) -> Option<&str> {
        self.reference_id.as_deref()
    }
    pub fn get_failure_code(&self) -> Option<&str> {
        self.failure_code.as_deref()
    }
    pub fn get_refund_fee_amount(&self) -> Option<f64> {
        self.refund_fee_amount
    }
    pub fn get_created(&self) -> Option<&str> {
        self.created.as_deref()
    }
    pub fn get_updated(&self) -> Option<&str> {
        self.updated.as_deref()
    }
    pub fn get_metadata(&self) -> Option<&HashMap<String, serde_json::Value>> {
        self.metadata.as_ref()
    }
}