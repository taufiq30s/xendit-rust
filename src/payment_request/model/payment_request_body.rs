use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    common::Currency,
    payment_method::PaymentMethodBody,
    payment_request::{ChannelProperties, ShippingInformation},
};

use super::BasketItem;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CaptureMethod {
    Automatic,
    Manual,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Initiator {
    Customer,
    Merchant,
}

#[skip_serializing_none]
#[derive(Serialize, Clone)]
pub struct PaymentRequestBody {
    currency: Currency,
    reference_id: Option<String>,
    amount: Option<f64>,
    payment_method: Option<PaymentMethodBody>,
    description: Option<String>,
    capture_method: Option<CaptureMethod>,
    initiator: Option<Initiator>,
    payment_method_id: Option<String>,
    channel_properties: Option<ChannelProperties>,
    shipping_information: Option<ShippingInformation>,
    items: Option<Vec<BasketItem>>,
    customer_id: Option<String>,
    customer: Option<HashMap<String, serde_json::Value>>,
    metadata: Option<HashMap<String, serde_json::Value>>,
}
impl PaymentRequestBody {
    pub fn new(currency: Currency) -> Self {
        Self {
            currency,
            reference_id: None,
            amount: None,
            payment_method: None,
            description: None,
            capture_method: None,
            initiator: None,
            payment_method_id: None,
            channel_properties: None,
            shipping_information: None,
            items: None,
            customer_id: None,
            customer: None,
            metadata: None,
        }
    }
    pub fn set_currency(&mut self, currency: Currency) -> &mut Self {
        self.currency = currency;
        self
    }
    pub fn get_currency(&self) -> &Currency {
        &self.currency
    }
    pub fn set_reference_id(&mut self, reference_id: String) -> &mut Self {
        self.reference_id = Some(reference_id);
        self
    }
    pub fn get_reference_id(&self) -> Option<&String> {
        self.reference_id.as_ref()
    }
    pub fn set_amount(&mut self, amount: f64) -> &mut Self {
        self.amount = Some(amount);
        self
    }
    pub fn get_amount(&self) -> Option<&f64> {
        self.amount.as_ref()
    }
    pub fn set_payment_method(&mut self, payment_method: PaymentMethodBody) -> &mut Self {
        self.payment_method = Some(payment_method);
        self
    }
    pub fn get_payment_method(&self) -> Option<&PaymentMethodBody> {
        self.payment_method.as_ref()
    }
    pub fn set_description(&mut self, description: String) -> &mut Self {
        self.description = Some(description);
        self
    }
    pub fn get_description(&self) -> Option<&String> {
        self.description.as_ref()
    }
    pub fn set_capture_method(&mut self, capture_method: CaptureMethod) -> &mut Self {
        self.capture_method = Some(capture_method);
        self
    }
    pub fn get_capture_method(&self) -> Option<&CaptureMethod> {
        self.capture_method.as_ref()
    }
    pub fn set_initiator(&mut self, initiator: Initiator) -> &mut Self {
        self.initiator = Some(initiator);
        self
    }
    pub fn get_initiator(&self) -> Option<&Initiator> {
        self.initiator.as_ref()
    }
    pub fn set_payment_method_id(&mut self, payment_method_id: String) -> &mut Self {
        self.payment_method_id = Some(payment_method_id);
        self
    }
    pub fn get_payment_method_id(&self) -> Option<&String> {
        self.payment_method_id.as_ref()
    }
    pub fn set_channel_properties(&mut self, channel_properties: ChannelProperties) -> &mut Self {
        self.channel_properties = Some(channel_properties);
        self
    }
    pub fn get_channel_properties(&self) -> Option<&ChannelProperties> {
        self.channel_properties.as_ref()
    }
    pub fn set_shipping_information(
        &mut self,
        shipping_information: ShippingInformation,
    ) -> &mut Self {
        self.shipping_information = Some(shipping_information);
        self
    }
    pub fn get_shipping_information(&self) -> Option<&ShippingInformation> {
        self.shipping_information.as_ref()
    }
    pub fn set_items(&mut self, items: Vec<BasketItem>) -> &mut Self {
        self.items = Some(items);
        self
    }
    pub fn get_items(&self) -> Option<&Vec<BasketItem>> {
        self.items.as_ref()
    }
    pub fn set_customer_id(&mut self, customer_id: String) -> &mut Self {
        self.customer_id = Some(customer_id);
        self
    }
    pub fn get_customer_id(&self) -> Option<&String> {
        self.customer_id.as_ref()
    }
    pub fn set_customer(&mut self, customer: HashMap<String, serde_json::Value>) -> &mut Self {
        self.customer = Some(customer);
        self
    }
    pub fn get_customer(&self) -> Option<&HashMap<String, serde_json::Value>> {
        self.customer.as_ref()
    }
    pub fn set_metadata(&mut self, metadata: HashMap<String, serde_json::Value>) -> &mut Self {
        self.metadata = Some(metadata);
        self
    }
    pub fn get_metadata(&self) -> Option<&HashMap<String, serde_json::Value>> {
        self.metadata.as_ref()
    }
    pub fn build(&self) -> Self {
        self.clone()
    }
}
