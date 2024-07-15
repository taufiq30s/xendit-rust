use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{common::Currency, payment_method::QRCodeChannel};

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone)]
pub struct QRCodeChannelProperties {
    pub qr_string: Option<String>,
    pub expires_at: Option<DateTime<Utc>>
}
impl QRCodeChannelProperties {
    pub fn new(
    ) -> Self {
        Self {
            qr_string: None,
            expires_at: None
        }
    }
    pub fn get_qr_string(&self) -> Option<&String> {
        self.qr_string.as_ref()
    }
    pub fn set_qr_string(&mut self, qr_string: Option<String>) -> &mut Self {
        self.qr_string = qr_string;
        self
    }
    pub fn get_expires_at(&self) -> Option<&DateTime<Utc>> {
        self.expires_at.as_ref()
    }
    pub fn set_expires_at(&mut self, expires_at: Option<DateTime<Utc>>) -> &mut Self {
        self.expires_at = expires_at;
        self
    }
    pub fn build(&mut self) -> Self {
        self.clone()
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct QRCodeParameter {
    pub amount: Option<f64>,
    pub currency: Option<Currency>,
    pub channel_code: Option<QRCodeChannel>,
    pub channel_properties: Option<QRCodeChannelProperties>
}
impl QRCodeParameter {
    pub fn new(
    ) -> Self {
        Self {
            amount: None,
            currency: None,
            channel_code: None,
            channel_properties: None
        }
    }
    pub fn get_amount(&self) -> Option<&f64> {
        self.amount.as_ref()
    }
    pub fn set_amount(&mut self, amount: Option<f64>) -> &mut Self {
        self.amount = amount;
        self
    }
    pub fn get_currency(&self) -> Option<&Currency> {
        self.currency.as_ref()
    }
    pub fn set_currency(&mut self, currency: Option<Currency>) -> &mut Self {
        self.currency = currency;
        self
    }
    pub fn get_channel_code(&self) -> Option<&QRCodeChannel> {
        self.channel_code.as_ref()
    }
    pub fn set_channel_code(&mut self, channel_code: Option<QRCodeChannel>) -> &mut Self {
        self.channel_code = channel_code;
        self
    }
    pub fn get_channel_properties(&self) -> Option<&QRCodeChannelProperties> {
        self.channel_properties.as_ref()
    }
    pub fn set_channel_properties(&mut self, channel_properties: Option<QRCodeChannelProperties>) -> &mut Self {
        self.channel_properties = channel_properties;
        self
    }
    pub fn build(&mut self) -> Self {
        self.clone()
    }
}