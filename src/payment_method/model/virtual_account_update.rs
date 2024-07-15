use serde::Serialize;
use serde_with::skip_serializing_none;

use super::VirtualAccountChannelProperties;

#[skip_serializing_none]
#[derive(Serialize, Clone)]
pub struct VirtualAccountParameterUpdate {
    pub amount: Option<f64>,
    pub min_amount: Option<f64>,
    pub max_amount: Option<f64>,
    pub channel_properties: Option<VirtualAccountChannelProperties>,
    pub alternative_display_types: Option<Vec<String>>
}
impl VirtualAccountParameterUpdate {
    pub fn new() -> Self {
        Self {
            amount: None,
            min_amount: None,
            max_amount: None,
            channel_properties: None,
            alternative_display_types: None
        }
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
    pub fn get_channel_properties(&self) -> Option<&VirtualAccountChannelProperties> {
        self.channel_properties.as_ref()
    }
    pub fn set_channel_properties(&mut self, channel_properties: VirtualAccountChannelProperties) -> &mut Self {
        self.channel_properties = Some(channel_properties);
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