use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

// Identity Account Request
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum IdentityAccountType {
    BankAccount,
    Ewallet,
    CreditCard,
    PayLater,
    Otc,
    QrCode
}

#[derive(Serialize, Deserialize, Clone)]
pub struct IdentityAccountRequestProperties {
    account_number: String,
    account_holder_name: Option<String>,
    swift_code: Option<String>,
    account_type: Option<String>,
    account_details: Option<String>,
    currency: Option<String>,
    token_id: Option<String>,
    account_id: Option<String>,
    payment_code: Option<String>,
    expires_at: Option<String>,
    qr_string: Option<String>,
}
impl IdentityAccountRequestProperties {
    pub fn new(account_number: String) -> Self {
        IdentityAccountRequestProperties {
            account_number,
            account_holder_name: None,
            swift_code: None,
            account_type: None,
            account_details: None,
            currency: None,
            token_id: None,
            account_id: None,
            payment_code: None,
            expires_at: None,
            qr_string: None
        }
    }
    pub fn set_account_number(&mut self, account_number: String) -> &mut Self {
        self.account_number = account_number;
        self
    }
    pub fn get_account_number(&self) -> &String {
        &self.account_number
    }
    pub fn set_account_holder_name(&mut self, account_holder_name: String) -> &mut Self {
        self.account_holder_name = Some(account_holder_name);
        self
    }
    pub fn get_account_holder_name(&self) -> Option<&String> {
        self.account_holder_name.as_ref()
    }
    pub fn set_swift_code(&mut self, swift_code: String) -> &mut Self {
        self.swift_code = Some(swift_code);
        self
    }
    pub fn get_swift_code(&self) -> Option<&String> {
        self.swift_code.as_ref()
    }
    pub fn set_account_type(&mut self, account_type: String) -> &mut Self {
        self.account_type = Some(account_type);
        self
    }
    pub fn get_account_type(&self) -> Option<&String> {
        self.account_type.as_ref()
    }
    pub fn set_account_details(&mut self, account_details: String) -> &mut Self {
        self.account_details = Some(account_details);
        self
    }
    pub fn get_account_details(&self) -> Option<&String> {
        self.account_details.as_ref()
    }
    pub fn set_currency(&mut self, currency: String) -> &mut Self {
        self.currency = Some(currency);
        self
    }
    pub fn get_currency(&self) -> Option<&String> {
        self.currency.as_ref()
    }
    pub fn set_token_id(&mut self, token_id: String) -> &mut Self {
        self.token_id = Some(token_id);
        self
    }
    pub fn get_token_id(&self) -> Option<&String> {
        self.token_id.as_ref()
    }
    pub fn set_account_id(&mut self, account_id: String) -> &mut Self {
        self.account_id = Some(account_id);
        self
    }
    pub fn get_account_id(&self) -> Option<&String> {
        self.account_id.as_ref()
    }
    pub fn set_payment_code(&mut self, payment_code: String) -> &mut Self {
        self.payment_code = Some(payment_code);
        self
    }
    pub fn get_payment_code(&self) -> Option<&String> {
        self.payment_code.as_ref()
    }
    pub fn set_expires_at(&mut self, expires_at: String) -> &mut Self {
        self.expires_at = Some(expires_at);
        self
    }
    pub fn get_expires_at(&self) -> Option<&String> {
        self.expires_at.as_ref()
    }
    pub fn set_qr_string(&mut self, qr_string: String) -> &mut Self {
        self.qr_string = Some(qr_string);
        self
    }
    pub fn get_qr_string(&self) -> Option<&String> {
        self.qr_string.as_ref()
    }
    pub fn build(&mut self) -> IdentityAccountRequestProperties {
        self.clone()
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone)]
pub struct IdentityAccountRequest {
    r#type: IdentityAccountType,
    company: Option<String>,
    description: Option<String>,
    country: Option<String>,
    properties: Option<IdentityAccountRequestProperties>
}
impl IdentityAccountRequest {
    pub fn new(r#type: IdentityAccountType) -> Self {
        IdentityAccountRequest {
            r#type,
            company: None,
            description: None,
            country: None,
            properties: None
        }
    }
    pub fn set_type(&mut self, r#type: IdentityAccountType) -> &mut Self {
        self.r#type = r#type;
        self
    }
    pub fn get_type(&self) -> &IdentityAccountType {
        &self.r#type
    }
    pub fn set_company(&mut self, company: String) -> &mut Self {
        self.company = Some(company);
        self
    }
    pub fn get_company(&self) -> Option<&String> {
        self.company.as_ref()
    }
    pub fn set_description(&mut self, description: String) -> &mut Self {
        self.description = Some(description);
        self
    }
    pub fn get_description(&self) -> Option<&String> {
        self.description.as_ref()
    }
    pub fn set_country(&mut self, country: String) -> &mut Self {
        self.country = Some(country);
        self
    }
    pub fn get_country(&self) -> Option<&String> {
        self.country.as_ref()
    }
    pub fn set_properties(&mut self, properties: IdentityAccountRequestProperties) -> &mut Self {
        self.properties = Some(properties);
        self
    }
    pub fn get_properties(&self) -> Option<&IdentityAccountRequestProperties> {
        self.properties.as_ref()
    }
    pub fn build(&mut self) -> IdentityAccountRequest {
        self.clone()
    }
}