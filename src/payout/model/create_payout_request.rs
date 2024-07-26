use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::common::Currency;

use super::{DigitalPayoutChannelProperties, ReceiptNotification};

#[skip_serializing_none]
#[derive(Serialize, Clone)]
pub struct CreatePayoutRequest {
    reference_id: String,
    channel_code: String,
    channel_properties: DigitalPayoutChannelProperties,
    amount: f32,
    currency: Currency,
    description: Option<String>,
    receipt_notification: Option<ReceiptNotification>,
    metadata: Option<serde_json::Value>,
}
impl CreatePayoutRequest {
    pub fn new(
        reference_id: String,
        channel_code: String,
        channel_properties: DigitalPayoutChannelProperties,
        amount: f32,
        currency: Currency,
    ) -> Self {
        Self {
            reference_id,
            channel_code,
            channel_properties,
            amount,
            currency,
            description: None,
            receipt_notification: None,
            metadata: None,
        }
    }
    pub fn get_reference_id(&self) -> &str {
        &self.reference_id
    }
    pub fn set_reference_id(&mut self, reference_id: String) {
        self.reference_id = reference_id;
    }
    pub fn get_channel_code(&self) -> &str {
        &self.channel_code
    }
    pub fn set_channel_code(&mut self, channel_code: String) {
        self.channel_code = channel_code;
    }
    pub fn get_channel_properties(&self) -> &DigitalPayoutChannelProperties {
        &self.channel_properties
    }
    pub fn set_channel_properties(&mut self, channel_properties: DigitalPayoutChannelProperties) {
        self.channel_properties = channel_properties;
    }
    pub fn get_amount(&self) -> f32 {
        self.amount
    }
    pub fn set_amount(&mut self, amount: f32) {
        self.amount = amount;
    }
    pub fn get_currency(&self) -> &Currency {
        &self.currency
    }
    pub fn set_currency(&mut self, currency: Currency) {
        self.currency = currency;
    }
    pub fn get_description(&self) -> Option<&str> {
        self.description.as_deref()
    }
    pub fn set_description(&mut self, description: String) {
        self.description = Some(description);
    }
    pub fn get_receipt_notification(&self) -> Option<&ReceiptNotification> {
        self.receipt_notification.as_ref()
    }
    pub fn set_receipt_notification(&mut self, receipt_notification: ReceiptNotification) {
        self.receipt_notification = Some(receipt_notification);
    }
    pub fn get_metadata(&self) -> Option<&serde_json::Value> {
        self.metadata.as_ref()
    }
    pub fn set_metadata(&mut self, metadata: serde_json::Value) {
        self.metadata = Some(metadata);
    }
    pub fn build(&self) -> Self {
        self.clone()
    }
}
