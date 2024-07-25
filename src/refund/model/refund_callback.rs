use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::{common::Currency, payment_method::PaymentCountry};

use super::RefundReason;

#[derive(Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PaymentMethodType {
    Card,
    DirectDebit,
    Ewallet
}

#[derive(Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RefundStatus {
    RequiresAction,
    Succeeded,
    Failed,
    Pending,
    Cancelled
}

#[derive(Deserialize)]
pub struct RefundCallbackData {
    id: String,
    payment_id: String,
    payment_method_type: PaymentMethodType,
    amount: f64,
    channel_code: String,
    status: RefundStatus,
    reason: RefundReason,
    country: PaymentCountry,
    currency: Currency,
    created: DateTime<Utc>,
    updated: DateTime<Utc>,
    invoice_id: Option<String>,
    reference_id: Option<String>,
    failure_code: Option<String>,
    refund_fee_amount: Option<f64>,
    metadata: Option<HashMap<String, serde_json::Value>>,
}
impl RefundCallbackData {
    pub fn get_id(&self) -> &str {
        &self.id
    }
    pub fn get_payment_id(&self) -> &str {
        &self.payment_id
    }
    pub fn get_payment_method_type(&self) -> &PaymentMethodType {
        &self.payment_method_type
    }
    pub fn get_amount(&self) -> f64 {
        self.amount
    }
    pub fn get_channel_code(&self) -> &str {
        &self.channel_code
    }
    pub fn get_status(&self) -> &RefundStatus {
        &self.status
    }
    pub fn get_reason(&self) -> &RefundReason {
        &self.reason
    }
    pub fn get_country(&self) -> &PaymentCountry {
        &self.country
    }
    pub fn get_currency(&self) -> &Currency {
        &self.currency
    }
    pub fn get_created(&self) -> &DateTime<Utc> {
        &self.created
    }
    pub fn get_updated(&self) -> &DateTime<Utc> {
        &self.updated
    }
    pub fn get_invoice_id(&self) -> Option<&str> {
        self.invoice_id.as_deref()
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
    pub fn get_metadata(&self) -> Option<&HashMap<String, serde_json::Value>> {
        self.metadata.as_ref()
    }
}

#[derive(Deserialize)]
pub struct RefundCallback {
    event: String,
    business_id: String,
    created: DateTime<Utc>,
    data: RefundCallbackData,
}

impl RefundCallback {
    pub fn get_event(&self) -> &str {
        &self.event
    }
    pub fn get_business_id(&self) -> &str {
        &self.business_id
    }
    pub fn get_created(&self) -> &DateTime<Utc> {
        &self.created
    }
    pub fn get_data(&self) -> &RefundCallbackData {
        &self.data
    }
}