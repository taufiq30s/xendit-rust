use std::collections::HashMap;

use serde::Deserialize;

use crate::{payment_method::PaymentMethod, payment_request::ChannelProperties};

#[derive(Deserialize)]
pub struct PaymentCallbackData {
    id: String,
    payment_request_id: Option<String>,
    reference_id: String,
    customer_id: Option<String>,
    currency: String,
    amount: f64,
    country: String,
    status: String,
    payment_method: PaymentMethod,
    channel_properties: Option<ChannelProperties>,
    payment_detail: Option<HashMap<String, serde_json::Value>>,
    failure_code: Option<String>,
    created: String,
    updated: String,
    metadata: Option<HashMap<String, serde_json::Value>>,
}

impl PaymentCallbackData {
    pub fn get_id(&self) -> &String {
        &self.id
    }
    pub fn get_payment_request_id(&self) -> Option<&String> {
        self.payment_request_id.as_ref()
    }
    pub fn get_reference_id(&self) -> &String {
        &self.reference_id
    }
    pub fn get_customer_id(&self) -> Option<&String> {
        self.customer_id.as_ref()
    }
    pub fn get_currency(&self) -> &String {
        &self.currency
    }
    pub fn get_amount(&self) -> &f64 {
        &self.amount
    }
    pub fn get_country(&self) -> &String {
        &self.country
    }
    pub fn get_status(&self) -> &String {
        &self.status
    }
    pub fn get_payment_method(&self) -> &PaymentMethod {
        &self.payment_method
    }
    pub fn get_channel_properties(&self) -> Option<&ChannelProperties> {
        self.channel_properties.as_ref()
    }
    pub fn get_payment_detail(&self) -> Option<&HashMap<String, serde_json::Value>> {
        self.payment_detail.as_ref()
    }
    pub fn get_failure_code(&self) -> Option<&String> {
        self.failure_code.as_ref()
    }
    pub fn get_created(&self) -> &String {
        &self.created
    }
    pub fn get_updated(&self) -> &String {
        &self.updated
    }
    pub fn get_metadata(&self) -> Option<&HashMap<String, serde_json::Value>> {
        self.metadata.as_ref()
    }
}

#[derive(Deserialize)]
pub struct PaymentCallback {
    event: String,
    business_id: String,
    created: String,
    data: PaymentCallbackData,
}
impl PaymentCallback {
    pub fn get_event(&self) -> &String {
        &self.event
    }
    pub fn get_business_id(&self) -> &String {
        &self.business_id
    }
    pub fn get_created(&self) -> &String {
        &self.created
    }
    pub fn get_data(&self) -> &PaymentCallbackData {
        &self.data
    }
}