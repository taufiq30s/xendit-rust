use serde::Serialize;

use crate::common::Currency;

use super::CardChannelProperties;

#[derive(Serialize, Clone)]
pub struct CardInformation {
    card_number: String,
    expiry_month: String,
    expiry_year: String,
    cardholder_name: Option<String>,
    cvv: Option<String>
}
impl CardInformation {
    pub(super) fn new(
        card_number: String,
        expiry_month: String,
        expiry_year: String
    ) -> Self {
        Self {
            card_number,
            expiry_month,
            expiry_year,
            cardholder_name: None,
            cvv: None
        }
    }
    pub fn get_card_number(&self) -> &str {
        &self.card_number
    }
    pub fn set_card_number(&mut self, card_number: String) -> &mut Self {
        self.card_number = card_number;
        self
    }
    pub fn get_expiry_month(&self) -> &str {
        &self.expiry_month
    }
    pub fn set_expiry_month(&mut self, expiry_month: String) -> &mut Self {
        self.expiry_month = expiry_month;
        self
    }
    pub fn get_expiry_year(&self) -> &str {
        &self.expiry_year
    }
    pub fn set_expiry_year(&mut self, expiry_year: String) -> &mut Self {
        self.expiry_year = expiry_year;
        self
    }
    pub fn get_cardholder_name(&self) -> Option<&String> {
        self.cardholder_name.as_ref()
    }
    pub fn set_cardholder_name(&mut self, cardholder_name: String) -> &mut Self {
        self.cardholder_name = Some(cardholder_name);
        self
    }
    pub fn get_cvv(&self) -> Option<&String> {
        self.cvv.as_ref()
    }
    pub fn set_cvv(&mut self, cvv: String) -> &mut Self {
        self.cvv = Some(cvv);
        self
    }
    pub fn build(&mut self) -> Self {
        self.clone()
    }
}

#[derive(Serialize, Clone)]
pub struct CardParameter {
    currency: Currency,
    channel_properties: CardChannelProperties,
    card_information: CardInformation
}
impl CardParameter {
    pub fn new(
        currency: Currency,
        card_number: String,
        expiry_month: String,
        expiry_year: String
    ) -> Self {
        Self {
            currency,
            channel_properties: CardChannelProperties::new(),
            card_information: CardInformation::new(card_number, expiry_month, expiry_year)
        }
    }
    pub fn get_currency(&self) -> &Currency {
        &self.currency
    }
    pub fn set_currency(&mut self, currency: Currency) -> &mut Self {
        self.currency = currency;
        self
    }
    pub fn get_channel_properties(&self) -> &CardChannelProperties {
        &self.channel_properties
    }
    pub fn set_channel_properties(&mut self, channel_properties: CardChannelProperties) -> &mut Self {
        self.channel_properties = channel_properties;
        self
    }
    pub fn get_card_information(&self) -> &CardInformation {
        &self.card_information
    }
    pub fn set_card_information(&mut self, card_information: CardInformation) -> &mut Self {
        self.card_information = card_information;
        self
    }
    pub fn set_cardholder_name(&mut self, cardholder_name: String) -> &mut Self {
        self.card_information.set_cardholder_name(cardholder_name);
        self
    }
    pub fn set_cvv(&mut self, cvv: String) -> &mut Self {
        self.card_information.set_cvv(cvv);
        self
    }
    pub fn build(&mut self) -> Self {
        self.clone()
    }
}