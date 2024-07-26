use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::common::Currency;

#[derive(Serialize, Clone)]
pub struct GetBalanceRequestParams {
    account_type: Option<String>,
    currency: Option<Currency>,
    at_timestamp: Option<DateTime<Utc>>,
}
impl GetBalanceRequestParams {
    pub fn new() -> Self {
        Self {
            account_type: Some(String::from("CASH")),
            currency: None,
            at_timestamp: None,
        }
    }
    pub fn get_currency(&self) -> Option<&Currency> {
        self.currency.as_ref()
    }
    pub fn set_currency(&mut self, currency: Currency) -> &mut Self {
        self.currency = Some(currency);
        self
    }
    pub fn get_at_timestamp(&self) -> Option<&DateTime<Utc>> {
        self.at_timestamp.as_ref()
    }
    pub fn set_at_timestamp(&mut self, at_timestamp: DateTime<Utc>) -> &mut Self {
        self.at_timestamp = Some(at_timestamp);
        self
    }
    pub fn get_account_type(&self) -> Option<&str> {
        self.account_type.as_deref()
    }
    pub fn set_account_type(&mut self, account_type: &str) -> &mut Self {
        self.account_type = Some(account_type.to_string());
        self
    }
    pub fn build(&self) -> Self {
        self.clone()
    }
}