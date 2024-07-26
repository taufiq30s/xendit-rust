use std::collections::HashMap;

use serde::Deserialize;
use chrono::{DateTime, Utc};

use crate::common::Currency;

use super::{DigitalPayoutChannelProperties, ReceiptNotification};

#[derive(Deserialize)]
pub struct GetPayouts200ResponseDataInner {
    reference_id: String,
    channel_code: String,
    channel_properties: DigitalPayoutChannelProperties,
    amount: f32,
    currency: Currency,
    id: String,
    created: DateTime<Utc>,
    updated: DateTime<Utc>,
    business_id: String,
    status: String,
    description: Option<String>,
    receipt_notification: Option<ReceiptNotification>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    failure_code: Option<String>,
    estimated_arrival_time: Option<DateTime<Utc>>,
}
impl GetPayouts200ResponseDataInner {
    pub fn get_reference_id(&self) -> &str {
        &self.reference_id
    }
    pub fn get_channel_code(&self) -> &str {
        &self.channel_code
    }
    pub fn get_channel_properties(&self) -> &DigitalPayoutChannelProperties {
        &self.channel_properties
    }
    pub fn get_amount(&self) -> f32 {
        self.amount
    }
    pub fn get_currency(&self) -> &Currency {
        &self.currency
    }
    pub fn get_id(&self) -> &str {
        &self.id
    }
    pub fn get_created(&self) -> &DateTime<Utc> {
        &self.created
    }
    pub fn get_updated(&self) -> &DateTime<Utc> {
        &self.updated
    }
    pub fn get_business_id(&self) -> &str {
        &self.business_id
    }
    pub fn get_status(&self) -> &str {
        &self.status
    }
    pub fn get_description(&self) -> Option<&str> {
        self.description.as_deref()
    }
    pub fn get_receipt_notification(&self) -> Option<&ReceiptNotification> {
        self.receipt_notification.as_ref()
    }
    pub fn get_metadata(&self) -> Option<&HashMap<String, serde_json::Value>> {
        self.metadata.as_ref()
    }
    pub fn get_failure_code(&self) -> Option<&str> {
        self.failure_code.as_deref()
    }
    pub fn get_estimated_arrival_time(&self) -> Option<&DateTime<Utc>> {
        self.estimated_arrival_time.as_ref()
    }
}
