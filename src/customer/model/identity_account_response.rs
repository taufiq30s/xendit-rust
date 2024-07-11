use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct IdentityAccountResponseProperties {
    account_number: Option<String>,
    account_holder_name: Option<String>,
    swift_code: Option<String>,
    account_type: Option<String>,
    account_details: Option<String>,
    currency: Option<String>,
    token_id: Option<String>,
    payment_code: Option<String>,
    expires_at: Option<String>,
    qr_string: Option<String>,
    account_id: Option<String>,
}
impl IdentityAccountResponseProperties {
    pub fn get_account_number(&self) -> Option<&String> {
        self.account_number.as_ref()
    }
    pub fn get_account_holder_name(&self) -> Option<&String> {
        self.account_holder_name.as_ref()
    }
    pub fn get_swift_code(&self) -> Option<&String> {
        self.swift_code.as_ref()
    }
    pub fn get_account_type(&self) -> Option<&String> {
        self.account_type.as_ref()
    }
    pub fn get_account_details(&self) -> Option<&String> {
        self.account_details.as_ref()
    }
    pub fn get_currency(&self) -> Option<&String> {
        self.currency.as_ref()
    }
    pub fn get_token_id(&self) -> Option<&String> {
        self.token_id.as_ref()
    }
    pub fn get_payment_code(&self) -> Option<&String> {
        self.payment_code.as_ref()
    }
    pub fn get_expires_at(&self) -> Option<&String> {
        self.expires_at.as_ref()
    }
    pub fn get_qr_string(&self) -> Option<&String> {
        self.qr_string.as_ref()
    }
    pub fn get_account_id(&self) -> Option<&String> {
        self.account_id.as_ref()
    }
}

#[derive(Deserialize)]
pub struct IdentityAccountResponse {
    id: Option<String>,
    code: Option<String>,
    company: String,
    description: String,
    country: String,
    holder_name: Option<String>,
    r#type: String,
    properties: IdentityAccountResponseProperties,
    created: Option<DateTime<Utc>>,
}

impl IdentityAccountResponse {
    pub fn get_id(&self) -> Option<&String> {
        self.id.as_ref()
    }
    pub fn get_code(&self) -> Option<&String> {
        self.code.as_ref()
    }
    pub fn get_company(&self) -> &str {
        &self.company
    }
    pub fn get_description(&self) -> &str {
        &self.description
    }
    pub fn get_country(&self) -> &str {
        &self.country
    }
    pub fn get_holder_name(&self) -> Option<&String> {
        self.holder_name.as_ref()
    }
    pub fn get_type(&self) -> &str {
        &self.r#type
    }
    pub fn get_properties(&self) -> &IdentityAccountResponseProperties {
        &self.properties
    }
    pub fn get_created(&self) -> Option<&DateTime<Utc>> {
        self.created.as_ref()
    }
}