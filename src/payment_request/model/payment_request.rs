use serde::Deserialize;
use std::collections::HashMap;

use crate::{
    common::Currency, payment_method::{PaymentCountry, PaymentMethod}, payment_request::{CaptureMethod, Initiator, ShippingInformation}
};

use super::{action::Action, BasketItem, CardVerificationResult};

#[derive(Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PaymentRequestStatus {
    Pending,
    RequiresAction,
    Canceled,
    Succeeded,
    Failed,
    Voided,
    Unknown,
    AwaitingCapture,
    Expired,
}

#[derive(Deserialize)]
pub struct PaymentRequest {
    id: String,
    created: String,
    updated: String,
    reference_id: String,
    business_id: String,
    currency: Currency,
    payment_method: PaymentMethod,
    status: PaymentRequestStatus,
    customer_id: Option<String>,
    customer: Option<HashMap<String, serde_json::Value>>,
    amount: Option<f64>,
    min_amount: Option<f64>,
    max_amount: Option<f64>,
    country: Option<PaymentCountry>,
    description: Option<String>,
    failure_code: Option<String>,
    capture_method: Option<CaptureMethod>,
    initiator: Option<Initiator>,
    card_verification_results: Option<CardVerificationResult>,
    actions: Option<Vec<Action>>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    shipping_information: Option<ShippingInformation>,
    items: Option<Vec<BasketItem>>,
}

impl PaymentRequest {
    pub fn get_id(&self) -> &str {
        &self.id
    }
    pub fn get_created(&self) -> &str {
        &self.created
    }
    pub fn get_updated(&self) -> &str {
        &self.updated
    }
    pub fn get_reference_id(&self) -> &str {
        &self.reference_id
    }
    pub fn get_business_id(&self) -> &str {
        &self.business_id
    }
    pub fn get_currency(&self) -> &Currency {
        &self.currency
    }
    pub fn get_payment_method(&self) -> &PaymentMethod {
        &self.payment_method
    }
    pub fn get_status(&self) -> &PaymentRequestStatus {
        &self.status
    }
    pub fn get_customer_id(&self) -> Option<&str> {
        self.customer_id.as_deref()
    }
    pub fn get_customer(&self) -> Option<&HashMap<String, serde_json::Value>> {
        self.customer.as_ref()
    }
    pub fn get_amount(&self) -> Option<f64> {
        self.amount
    }
    pub fn get_min_amount(&self) -> Option<f64> {
        self.min_amount
    }
    pub fn get_max_amount(&self) -> Option<f64> {
        self.max_amount
    }
    pub fn get_country(&self) -> Option<&PaymentCountry> {
        self.country.as_ref()
    }
    pub fn get_description(&self) -> Option<&str> {
        self.description.as_deref()
    }
    pub fn get_failure_code(&self) -> Option<&str> {
        self.failure_code.as_deref()
    }
    pub fn get_capture_method(&self) -> Option<&CaptureMethod> {
        self.capture_method.as_ref()
    }
    pub fn get_initiator(&self) -> Option<&Initiator> {
        self.initiator.as_ref()
    }
    pub fn get_card_verification_results(&self) -> Option<&CardVerificationResult> {
        self.card_verification_results.as_ref()
    }
    pub fn get_actions(&self) -> Option<&Vec<Action>> {
        self.actions.as_ref()
    }
    pub fn get_metadata(&self) -> Option<&HashMap<String, serde_json::Value>> {
        self.metadata.as_ref()
    }
    pub fn get_shipping_information(&self) -> Option<&ShippingInformation> {
        self.shipping_information.as_ref()
    }
    pub fn get_items(&self) -> Option<&Vec<BasketItem>> {
        self.items.as_ref()
    }
}
