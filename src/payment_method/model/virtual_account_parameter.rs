use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{common::Currency, payment_method::VirtualAccountChannel};

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone)]
pub struct VirtualAccountChannelProperties {
    pub customer_name: Option<String>,
    pub virtual_account_number: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
    pub suggested_amount: Option<f64>
}
impl VirtualAccountChannelProperties {
    pub fn new() -> Self {
        Self {
            customer_name: None,
            virtual_account_number: None,
            expires_at: None,
            suggested_amount: None
        }
    }
    pub fn get_customer_name(&self) -> Option<&String> {
        self.customer_name.as_ref()
    }
    pub fn set_customer_name(&mut self, customer_name: String) -> &mut Self {
        self.customer_name = Some(customer_name);
        self
    }
    pub fn get_virtual_account_number(&self) -> Option<&String> {
        self.virtual_account_number.as_ref()
    }
    pub fn set_virtual_account_number(&mut self, virtual_account_number: String) -> &mut Self {
        self.virtual_account_number = Some(virtual_account_number);
        self
    }
    pub fn get_expires_at(&self) -> Option<&DateTime<Utc>> {
        self.expires_at.as_ref()
    }
    pub fn set_expires_at(&mut self, expires_at: DateTime<Utc>) -> &mut Self {
        self.expires_at = Some(expires_at);
        self
    }
    pub fn get_suggested_amount(&self) -> Option<&f64> {
        self.suggested_amount.as_ref()
    }
    pub fn set_suggested_amount(&mut self, suggested_amount: f64) -> &mut Self {
        self.suggested_amount = Some(suggested_amount);
        self
    }
    pub fn build(&self) -> VirtualAccountChannelProperties {
        self.clone()
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone)]
pub struct VirtualAccountParameter {
    pub channel_code: VirtualAccountChannel,
    pub channel_properties: VirtualAccountChannelProperties,
    pub amount: Option<f64>,
    pub min_amount: Option<f64>,
    pub max_amount: Option<f64>,
    pub currency: Option<Currency>,
    pub alternative_display_types: Option<Vec<String>>
}
impl VirtualAccountParameter {
    pub fn new(
        channel_code: VirtualAccountChannel
    ) -> Self {
        Self {
            channel_code,
            channel_properties: VirtualAccountChannelProperties::new(),
            amount: None,
            min_amount: None,
            max_amount: None,
            currency: Some(Currency::IDR),
            alternative_display_types: None
        }
    }
    pub fn get_channel_code(&self) -> &VirtualAccountChannel {
        &self.channel_code
    }
    pub fn set_channel_code(&mut self, channel_code: VirtualAccountChannel) -> &mut Self {
        self.channel_code = channel_code;
        self
    }
    pub fn get_channel_properties(&self) -> &VirtualAccountChannelProperties {
        &self.channel_properties
    }
    pub fn set_channel_properties(&mut self, channel_properties: VirtualAccountChannelProperties) -> &mut Self {
        self.channel_properties = channel_properties;
        self
    }
    pub fn get_amount(&self) -> Option<&f64> {
        self.amount.as_ref()
    }
    pub fn set_amount(&mut self, amount: f64) -> &mut Self {
        self.amount = Some(amount);
        self
    }
    pub fn get_min_amount(&self) -> Option<&f64> {
        self.min_amount.as_ref()
    }
    pub fn set_min_amount(&mut self, min_amount: f64) -> &mut Self {
        self.min_amount = Some(min_amount);
        self
    }
    pub fn get_max_amount(&self) -> Option<&f64> {
        self.max_amount.as_ref()
    }
    pub fn set_max_amount(&mut self, max_amount: f64) -> &mut Self {
        self.max_amount = Some(max_amount);
        self
    }
    pub fn get_currency(&self) -> Option<&Currency> {
        self.currency.as_ref()
    }
    pub fn set_currency(&mut self, currency: Currency) -> &mut Self {
        self.currency = Some(currency);
        self
    }
    pub fn get_alternative_display_types(&self) -> Option<&Vec<String>> {
        self.alternative_display_types.as_ref()
    }
    pub fn add_alternative_display_type(&mut self, alternative_display_type: String) -> &mut Self {
        if let Some(alternative_display_types) = &mut self.alternative_display_types {
            alternative_display_types.push(alternative_display_type);
        }
        self
    }
    pub fn build(&mut self) -> Self {
        self.clone()
    }
}