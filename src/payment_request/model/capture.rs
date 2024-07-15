use std::collections::HashMap;

use serde::Deserialize;

use crate::payment_method::PaymentMethod;

#[derive(Deserialize)]
pub struct Capture {
    id: String,
    payment_request_id: String,
    payment_id: String,
    reference_id: String,
    currency: String,
    authorized_amount: f64,
    captured_amount: f64,
    status: String,
    payment_method: PaymentMethod,
    failure_code: Option<String>,
    customer_id: Option<String>,
    metadata: HashMap<String, serde_json::Value>,
    channel_properties: HashMap<String, serde_json::Value>,
    created: String,
    updated: String,
}
impl Capture {
    pub fn get_id(&self) -> &String {
        &self.id
    }
    pub fn get_payment_request_id(&self) -> &String {
        &self.payment_request_id
    }
    pub fn get_payment_id(&self) -> &String {
        &self.payment_id
    }
    pub fn get_reference_id(&self) -> &String {
        &self.reference_id
    }
    pub fn get_currency(&self) -> &String {
        &self.currency
    }
    pub fn get_authorized_amount(&self) -> &f64 {
        &self.authorized_amount
    }
    pub fn get_captured_amount(&self) -> &f64 {
        &self.captured_amount
    }
    pub fn get_status(&self) -> &String {
        &self.status
    }
    pub fn get_payment_method(&self) -> &PaymentMethod {
        &self.payment_method
    }
    pub fn get_failure_code(&self) -> Option<&String> {
        self.failure_code.as_ref()
    }
    pub fn get_customer_id(&self) -> Option<&String> {
        self.customer_id.as_ref()
    }
    pub fn get_metadata(&self) -> &HashMap<String, serde_json::Value> {
        &self.metadata
    }
    pub fn get_channel_properties(&self) -> &HashMap<String, serde_json::Value> {
        &self.channel_properties
    }
    pub fn get_created(&self) -> &String {
        &self.created
    }
    pub fn get_updated(&self) -> &String {
        &self.updated
    }
}
