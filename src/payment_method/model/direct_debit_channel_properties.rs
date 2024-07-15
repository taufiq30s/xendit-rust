use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone)]
pub struct DirectDebitChannelProperties {
    success_return_url: Option<String>,
    failure_return_url: Option<String>,
    mobile_number: Option<String>,
    card_last_four: Option<String>,
    card_expiry: Option<String>,
    email: Option<String>,
    identity_document_number: Option<String>,
    require_auth: Option<bool>,
    account_number: Option<String>
}
impl DirectDebitChannelProperties {
    pub(super) fn new() -> Self {
        Self {
            success_return_url: None,
            failure_return_url: None,
            mobile_number: None,
            card_last_four: None,
            card_expiry: None,
            email: None,
            identity_document_number: None,
            require_auth: None,
            account_number: None
        }
    }
    pub fn get_success_return_url(&self) -> Option<&String> {
        self.success_return_url.as_ref()
    }
    pub fn set_success_return_url(&mut self, success_return_url: Option<String>) -> &mut Self {
        self.success_return_url = success_return_url;
        self
    }
    pub fn get_failure_return_url(&self) -> Option<&String> {
        self.failure_return_url.as_ref()
    }
    pub fn set_failure_return_url(&mut self, failure_return_url: Option<String>) -> &mut Self {
        self.failure_return_url = failure_return_url;
        self
    }
    pub fn get_mobile_number(&self) -> Option<&String> {
        self.mobile_number.as_ref()
    }
    pub fn set_mobile_number(&mut self, mobile_number: Option<String>) -> &mut Self {
        self.mobile_number = mobile_number;
        self
    }
    pub fn get_card_last_four(&self) -> Option<&String> {
        self.card_last_four.as_ref()
    }
    pub fn set_card_last_four(&mut self, card_last_four: Option<String>) -> &mut Self {
        self.card_last_four = card_last_four;
        self
    }
    pub fn get_card_expiry(&self) -> Option<&String> {
        self.card_expiry.as_ref()
    }
    pub fn set_card_expiry(&mut self, card_expiry: Option<String>) -> &mut Self {
        self.card_expiry = card_expiry;
        self
    }
    pub fn get_email(&self) -> Option<&String> {
        self.email.as_ref()
    }
    pub fn set_email(&mut self, email: Option<String>) -> &mut Self {
        self.email = email;
        self
    }
    pub fn get_identity_document_number(&self) -> Option<&String> {
        self.identity_document_number.as_ref()
    }
    pub fn set_identity_document_number(&mut self, identity_document_number: Option<String>) -> &mut Self {
        self.identity_document_number = identity_document_number;
        self
    }
    pub fn get_require_auth(&self) -> Option<&bool> {
        self.require_auth.as_ref()
    }
    pub fn set_require_auth(&mut self, require_auth: Option<bool>) -> &mut Self {
        self.require_auth = require_auth;
        self
    }
    pub fn get_account_number(&self) -> Option<&String> {
        self.account_number.as_ref()
    }
    pub fn set_account_number(&mut self, account_number: Option<String>) -> &mut Self {
        self.account_number = account_number;
        self
    }
    pub fn build(&self) -> Self {
        self.clone()
    }
}