use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ChannelCategory {
    Bank,
    Ewallet,
    Otc,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ChannelAmountLimits {
    minimum: f64,
    maximum: f64,
    minimum_increment: f64,
}
impl ChannelAmountLimits {
    pub fn new(minimum: f64, maximum: f64, minimum_increment: f64) -> Self {
        ChannelAmountLimits {
            minimum,
            maximum,
            minimum_increment,
        }
    }
    pub fn get_minimum(&self) -> f64 {
        self.minimum
    }
    pub fn set_minimum(&mut self, minimum: f64) {
        self.minimum = minimum;
    }
    pub fn get_maximum(&self) -> f64 {
        self.maximum
    }
    pub fn set_maximum(&mut self, maximum: f64) {
        self.maximum = maximum;
    }
    pub fn get_minimum_increment(&self) -> f64 {
        self.minimum_increment
    }
    pub fn set_minimum_increment(&mut self, minimum_increment: f64) {
        self.minimum_increment = minimum_increment;
    }
    pub fn build(&self) -> Self {
        self.clone()
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Channel {
    channel_code: String,
    channel_category: ChannelCategory,
    currency: String,
    channel_name: String,
    amount_limits: ChannelAmountLimits,
}
impl Channel {
    pub fn new(
        channel_code: String,
        channel_category: ChannelCategory,
        currency: String,
        channel_name: String,
        amount_limits: ChannelAmountLimits,
    ) -> Self {
        Channel {
            channel_code,
            channel_category,
            currency,
            channel_name,
            amount_limits,
        }
    }
    pub fn get_channel_code(&self) -> String {
        self.channel_code.clone()
    }
    pub fn set_channel_code(&mut self, channel_code: String) {
        self.channel_code = channel_code;
    }
    pub fn get_channel_category(&self) -> ChannelCategory {
        self.channel_category.clone()
    }
    pub fn set_channel_category(&mut self, channel_category: ChannelCategory) {
        self.channel_category = channel_category;
    }
    pub fn get_currency(&self) -> String {
        self.currency.clone()
    }
    pub fn set_currency(&mut self, currency: String) {
        self.currency = currency;
    }
    pub fn get_channel_name(&self) -> String {
        self.channel_name.clone()
    }
    pub fn set_channel_name(&mut self, channel_name: String) {
        self.channel_name = channel_name;
    }
    pub fn get_amount_limits(&self) -> ChannelAmountLimits {
        self.amount_limits.clone()
    }
    pub fn set_amount_limits(&mut self, amount_limits: ChannelAmountLimits) {
        self.amount_limits = amount_limits;
    }
    pub fn build(&self) -> Self {
        self.clone()
    }
}
