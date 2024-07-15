use serde::Deserialize;

use crate::payment_method::{CardChannelProperties, CardVerificationResults};

use super::TokenizedCardInformation;

#[derive(Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CardChannelCode {
    Gpn,
    UnknownEnumValue
}

#[derive(Deserialize)]
pub struct Card {
    channel_properties: Option<CardChannelProperties>,
    channel_code: Option<CardChannelCode>,
    currency: Option<String>,
    card_information: Option<TokenizedCardInformation>,
    card_verification_results: Option<CardVerificationResults>,
}

impl Card {
    pub fn get_channel_properties(&self) -> Option<&CardChannelProperties> {
        self.channel_properties.as_ref()
    }
    pub fn get_channel_code(&self) -> Option<&CardChannelCode> {
        self.channel_code.as_ref()
    }
    pub fn get_currency(&self) -> Option<&String> {
        self.currency.as_ref()
    }
    pub fn get_card_information(&self) -> Option<&TokenizedCardInformation> {
        self.card_information.as_ref()
    }
    pub fn get_card_verification_results(&self) -> Option<&CardVerificationResults> {
        self.card_verification_results.as_ref()
    }
}
