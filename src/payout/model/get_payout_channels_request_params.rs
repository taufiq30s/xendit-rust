use serde::Serialize;

use crate::common::Currency;

use super::ChannelCategory;

#[derive(Serialize, Clone)]
pub struct GetPayoutChannelsRequestParams {
    currency: Option<Currency>,
    channel_category: Option<Vec<ChannelCategory>>,
    channel_code: Option<String>,
}
impl GetPayoutChannelsRequestParams {
    pub fn new() -> Self {
        Self {
            currency: None,
            channel_category: None,
            channel_code: None,
        }
    }
    pub fn get_currency(&self) -> Option<&Currency> {
        self.currency.as_ref()
    }
    pub fn set_currency(&mut self, currency: Currency) -> &mut Self {
        self.currency = Some(currency);
        self
    }
    pub fn get_channel_category(&self) -> Option<&Vec<ChannelCategory>> {
        self.channel_category.as_ref()
    }
    pub fn set_channel_category(&mut self, channel_category: Vec<ChannelCategory>) -> &mut Self {
        self.channel_category = Some(channel_category);
        self
    }
    pub fn get_channel_code(&self) -> Option<&String> {
        self.channel_code.as_ref()
    }
    pub fn set_channel_code(&mut self, channel_code: String) -> &mut Self {
        self.channel_code = Some(channel_code);
        self
    }
    pub fn build(&self) -> Self {
        self.clone()
    }
}
