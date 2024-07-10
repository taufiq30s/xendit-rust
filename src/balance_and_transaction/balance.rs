use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};

use crate::client::XenditClient;

/// Get Balance
///
/// Retrieves balance for your business, defaults to CASH type

#[derive(Serialize)]
pub struct GetBalanceParams {
    account_type: Option<String>,
    currency: Option<String>,
    at_timestamp: Option<String>,
}
impl GetBalanceParams {
    pub fn new() -> Self {
        Self {
            account_type: Some(String::from("CASH")),
            currency: None,
            at_timestamp: None,
        }
    }
    pub fn set_currency(&mut self, currency: &str) -> &mut Self {
        self.currency = Some(currency.to_string());
        self
    }
    pub fn set_at_timestamp(&mut self, at_timestamp: &str) -> &mut Self {
        self.at_timestamp = Some(at_timestamp.to_string());
        self
    }
    pub fn set_account_type(&mut self, account_type: &str) -> &mut Self {
        self.account_type = Some(account_type.to_string());
        self
    }
    pub fn build(&self) -> GetBalanceParams {
        GetBalanceParams {
            account_type: self.account_type.clone(),
            currency: self.currency.clone(),
            at_timestamp: self.at_timestamp.clone(),
        }
    }
}

#[derive(Deserialize)]
pub struct BalanceModel {
    pub balance: u64,
}

pub struct BalanceClient<'a> {
    client: &'a XenditClient,
}
impl<'a> BalanceClient<'a> {
    pub fn new(client: &'a XenditClient) -> Self {
        BalanceClient { client }
    }
    fn process_custom_header(&self, for_user_id: Option<String>) -> Option<HeaderMap> {
        if for_user_id.is_none() {
            return None;
        }
        let mut headers = HeaderMap::new();
        headers.insert("for-user-id", for_user_id.unwrap().parse().unwrap());
        Some(headers)
    }
    pub async fn get_balance(
        &self,
        params: GetBalanceParams,
        for_user_id: Option<String>
    ) -> Result<u64, Box<dyn std::error::Error>> {
        let result = self
            .client
            .get_with_params::<BalanceModel, _>(
                "/balance",
                params,
                self.process_custom_header(for_user_id).as_ref()
            )
            .await?;
        Ok(result.balance)
    }
}
