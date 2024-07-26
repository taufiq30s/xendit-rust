use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::common::Currency;

#[derive(Deserialize)]
pub struct FeeResponse {
    xendit_fee: u64,
    value_added_tax: u64,
    xendit_withholding_tax: u64,
    third_party_withholding_tax: u64,
    status: String,
}
impl FeeResponse {
    pub fn get_xendit_fee(&self) -> u64 {
        self.xendit_fee
    }
    pub fn get_value_added_tax(&self) -> u64 {
        self.value_added_tax
    }
    pub fn get_xendit_withholding_tax(&self) -> u64 {
        self.xendit_withholding_tax
    }
    pub fn get_third_party_withholding_tax(&self) -> u64 {
        self.third_party_withholding_tax
    }
    pub fn get_status(&self) -> &str {
        &self.status
    }
}

#[derive(Deserialize)]
pub struct Transaction {
    id: String,
    product_id: String,
    r#type: String,
    channel_code: Option<String>,
    reference_id: Option<String>,
    account_identifier: Option<String>,
    currency: Option<Currency>,
    amount: u64,
    net_amount: u64,
    cashflow: String,
    status: String,
    channel_category: String,
    business_id: String,
    created: DateTime<Utc>,
    updated: DateTime<Utc>,
    fee: FeeResponse,
    settlement_status: Option<String>,
    estimated_settlement_time: Option<DateTime<Utc>>,
}
impl Transaction {
    pub fn get_id(&self) -> &str {
        &self.id
    }
    pub fn get_product_id(&self) -> &str {
        &self.product_id
    }
    pub fn get_type(&self) -> &str {
        &self.r#type
    }
    pub fn get_channel_code(&self) -> Option<&str> {
        self.channel_code.as_deref()
    }
    pub fn get_reference_id(&self) -> Option<&str> {
        self.reference_id.as_deref()
    }
    pub fn get_account_identifier(&self) -> Option<&str> {
        self.account_identifier.as_deref()
    }
    pub fn get_currency(&self) -> Option<&Currency> {
        self.currency.as_ref()
    }
    pub fn get_amount(&self) -> u64 {
        self.amount
    }
    pub fn get_net_amount(&self) -> u64 {
        self.net_amount
    }
    pub fn get_cashflow(&self) -> &str {
        &self.cashflow
    }
    pub fn get_status(&self) -> &str {
        &self.status
    }
    pub fn get_channel_category(&self) -> &str {
        &self.channel_category
    }
    pub fn get_business_id(&self) -> &str {
        &self.business_id
    }
    pub fn get_created(&self) -> DateTime<Utc> {
        self.created
    }
    pub fn get_updated(&self) -> DateTime<Utc> {
        self.updated
    }
    pub fn get_fee(&self) -> &FeeResponse {
        &self.fee
    }
    pub fn get_settlement_status(&self) -> Option<&str> {
        self.settlement_status.as_deref()
    }
    pub fn get_estimated_settlement_time(&self) -> Option<DateTime<Utc>> {
        self.estimated_settlement_time
    }
}