use serde::Deserialize;

use crate::{common::Currency, payment_method::VirtualAccountChannel};

use super::VirtualAccountChannelProperties;

#[derive(Deserialize)]
pub struct VirtualAccountAlternativeDisplay {
    r#type: Option<String>,
    data: Option<String>
}
impl VirtualAccountAlternativeDisplay {
    pub fn get_type(&self) -> Option<&String> {
        self.r#type.as_ref()
    }
    pub fn get_data(&self) -> Option<&String> {
        self.data.as_ref()
    }
}

#[derive(Deserialize)]
pub struct VirtualAccount {
    channel_code: VirtualAccountChannel,
    channel_properties: VirtualAccountChannelProperties,
    amount: Option<f64>,
    min_amount: Option<f64>,
    max_amount: Option<f64>,
    currency: Option<Currency>,
    alternative_display_types: Option<Vec<String>>,
    alternative_displays: Option<VirtualAccountAlternativeDisplay>
}
impl VirtualAccount {
    pub fn get_channel_code(&self) -> &VirtualAccountChannel {
        &self.channel_code
    }
    pub fn get_channel_properties(&self) -> &VirtualAccountChannelProperties {
        &self.channel_properties
    }
    pub fn get_amount(&self) -> Option<&f64> {
        self.amount.as_ref()
    }
    pub fn get_min_amount(&self) -> Option<&f64> {
        self.min_amount.as_ref()
    }
    pub fn get_max_amount(&self) -> Option<&f64> {
        self.max_amount.as_ref()
    }
    pub fn get_currency(&self) -> Option<&Currency> {
        self.currency.as_ref()
    }
    pub fn get_alternative_display_types(&self) -> Option<&Vec<String>> {
        self.alternative_display_types.as_ref()
    }
    pub fn get_alternative_displays(&self) -> Option<&VirtualAccountAlternativeDisplay> {
        self.alternative_displays.as_ref()
    }
}