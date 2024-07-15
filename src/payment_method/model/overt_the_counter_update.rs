use serde::Serialize;
use serde_with::skip_serializing_none;

use super::OverTheCounterChannelProperties;

#[skip_serializing_none]
#[derive(Serialize, Clone)]
pub struct OverTheCounterParameterUpdate {
    pub amount: Option<f64>,
    pub channel_properties: Option<OverTheCounterChannelProperties>,
}
impl OverTheCounterParameterUpdate {
    pub fn new() -> Self {
        Self {
            amount: None,
            channel_properties: None,
        }
    }
    pub fn get_amount(&self) -> Option<&f64> {
        self.amount.as_ref()
    }
    pub fn set_amount(&mut self, amount: f64) -> &mut Self {
        self.amount = Some(amount);
        self
    }
    pub fn get_channel_properties(&self) -> Option<&OverTheCounterChannelProperties> {
        self.channel_properties.as_ref()
    }
    pub fn set_channel_properties(
        &mut self,
        channel_properties: OverTheCounterChannelProperties,
    ) -> &mut Self {
        self.channel_properties = Some(channel_properties);
        self
    }
    pub fn build(&mut self) -> Self {
        self.clone()
    }
}
