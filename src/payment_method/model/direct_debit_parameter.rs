use serde::Serialize;

use crate::payment_method::{DirectDebitChannel, DirectDebitChannelProperties};

#[derive(Serialize, Clone)]
pub struct DirectDebitParameter {
    channel_code: DirectDebitChannel,
    channel_properties: DirectDebitChannelProperties
}
impl DirectDebitParameter {
    pub fn new(
        channel_code: DirectDebitChannel
    ) -> Self {
        Self {
            channel_code,
            channel_properties: DirectDebitChannelProperties::new()
        }
    }
    pub fn get_channel_code(&self) -> DirectDebitChannel {
        self.channel_code.clone()
    }
    pub fn set_channel_code(&mut self, channel_code: DirectDebitChannel) -> &mut Self {
        self.channel_code = channel_code;
        self
    }
    pub fn get_channel_properties(&self) -> &DirectDebitChannelProperties {
        &self.channel_properties
    }
    pub fn set_channel_properties(&mut self, channel_properties: DirectDebitChannelProperties) -> &mut Self {
        self.channel_properties = channel_properties;
        self
    }
    pub fn build(&mut self) -> Self {
        self.clone()
    }
}