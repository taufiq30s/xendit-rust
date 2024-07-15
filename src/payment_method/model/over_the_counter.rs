use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{common::Currency, payment_method::OverTheCounterChannel};

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone)]
pub struct OverTheCounterChannelProperties {
    customer_name: String,
    payment_code: Option<String>,
    expires_at: Option<DateTime<Utc>>,
}
impl OverTheCounterChannelProperties {
    pub(super) fn new(customer_name: String) -> Self {
        Self {
            customer_name,
            payment_code: None,
            expires_at: None,
        }
    }
    pub fn get_customer_name(&self) -> &String {
        &self.customer_name
    }
    pub fn set_customer_name(&mut self, customer_name: String) -> &mut Self {
        self.customer_name = customer_name;
        self
    }
    pub fn get_payment_code(&self) -> Option<&String> {
        self.payment_code.as_ref()
    }
    pub fn set_payment_code(&mut self, payment_code: String) -> &mut Self {
        self.payment_code = Some(payment_code);
        self
    }
    pub fn get_expires_at(&self) -> Option<&DateTime<Utc>> {
        self.expires_at.as_ref()
    }
    pub fn set_expires_at(&mut self, expires_at: DateTime<Utc>) -> &mut Self {
        self.expires_at = Some(expires_at);
        self
    }
    pub fn build(&mut self) -> Self {
        self.clone()
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct OverTheCounter {
    amount: Option<f64>,
    currency: Currency,
    channel_code: OverTheCounterChannel,
    channel_properties: OverTheCounterChannelProperties,
}
impl OverTheCounter {
    pub fn new(channel_code: OverTheCounterChannel, customer_name: String) -> Self {
        Self {
            amount: None,
            currency: Currency::IDR,
            channel_code,
            channel_properties: OverTheCounterChannelProperties::new(customer_name),
        }
    }
    pub fn get_amount(&self) -> Option<&f64> {
        self.amount.as_ref()
    }
    pub fn set_amount(&mut self, amount: f64) -> &mut Self {
        self.amount = Some(amount);
        self
    }
    pub fn get_currency(&self) -> &Currency {
        &self.currency
    }
    pub fn set_currency(&mut self, currency: Currency) -> &mut Self {
        self.currency = currency;
        self
    }
    pub fn get_channel_code(&self) -> &OverTheCounterChannel {
        &self.channel_code
    }
    pub fn set_channel_code(&mut self, channel_code: OverTheCounterChannel) -> &mut Self {
        self.channel_code = channel_code;
        self
    }
    pub fn get_channel_properties(&self) -> &OverTheCounterChannelProperties {
        &self.channel_properties
    }
    pub fn set_channel_properties(
        &mut self,
        channel_properties: OverTheCounterChannelProperties,
    ) -> &mut Self {
        self.channel_properties = channel_properties;
        self
    }
    pub fn build(&mut self) -> Self {
        self.clone()
    }
}
