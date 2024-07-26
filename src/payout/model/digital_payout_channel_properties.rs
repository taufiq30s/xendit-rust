use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ChannelAccountType {
    NationalId,
    MobileNo,
    Passport,
    BusinessRegistration,
    BankAccount,
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Clone)]
pub struct DigitalPayoutChannelProperties {
    account_number: String,
    account_holder_name: String,
    account_type: Option<ChannelAccountType>,
}
impl DigitalPayoutChannelProperties {
    pub fn new(account_number: String, account_holder_name: String) -> Self {
        Self {
            account_number,
            account_holder_name,
            account_type: None,
        }
    }
    pub fn get_account_number(&self) -> &str {
        &self.account_number
    }
    pub fn set_account_number(&mut self, account_number: String) {
        self.account_number = account_number;
    }
    pub fn get_account_holder_name(&self) -> &str {
        &self.account_holder_name
    }
    pub fn set_account_holder_name(&mut self, account_holder_name: String) {
        self.account_holder_name = account_holder_name;
    }
    pub fn get_account_type(&self) -> &Option<ChannelAccountType> {
        &self.account_type
    }
    pub fn set_account_type(&mut self, account_type: Option<ChannelAccountType>) {
        self.account_type = account_type;
    }
    pub fn build(&self) -> Self {
        self.clone()
    }
}
