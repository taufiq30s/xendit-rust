use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NetworkCard {
    Visa,
    Mastercard,
    Jcb,
    Amex,
    UnknownEnumValue,
}

#[derive(Deserialize)]
pub struct TokenizedCardInformation {
    token_id: Option<String>,
    masked_card_number: Option<String>,
    cardholder_name: Option<String>,
    expiry_month: Option<String>,
    expiry_year: Option<String>,
    fingerprint: Option<String>,
    type_: Option<String>,
    network: Option<NetworkCard>,
    country: Option<String>,
    issuer: Option<String>,
    card_number: Option<String>,
    one_time_token: Option<String>,
}

impl TokenizedCardInformation {
    pub fn get_token_id(&self) -> Option<&String> {
        self.token_id.as_ref()
    }
    pub fn get_masked_card_number(&self) -> Option<&String> {
        self.masked_card_number.as_ref()
    }
    pub fn get_cardholder_name(&self) -> Option<&String> {
        self.cardholder_name.as_ref()
    }
    pub fn get_expiry_month(&self) -> Option<&String> {
        self.expiry_month.as_ref()
    }
    pub fn get_expiry_year(&self) -> Option<&String> {
        self.expiry_year.as_ref()
    }
    pub fn get_fingerprint(&self) -> Option<&String> {
        self.fingerprint.as_ref()
    }
    pub fn get_type_(&self) -> Option<&String> {
        self.type_.as_ref()
    }
    pub fn get_network(&self) -> Option<&NetworkCard> {
        self.network.as_ref()
    }
    pub fn get_country(&self) -> Option<&String> {
        self.country.as_ref()
    }
    pub fn get_issuer(&self) -> Option<&String> {
        self.issuer.as_ref()
    }
    pub fn get_card_number(&self) -> Option<&String> {
        self.card_number.as_ref()
    }
    pub fn get_one_time_token(&self) -> Option<&String> {
        self.one_time_token.as_ref()
    }
}
