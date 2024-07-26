use reqwest::header::HeaderMap;
use serde::Deserialize;

use crate::client::XenditClient;

use super::model::GetBalanceRequestParams;

/// Get Balance
///
/// Retrieves balance for your business, defaults to CASH type
#[derive(Deserialize)]
pub struct Balance {
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
        params: GetBalanceRequestParams,
        for_user_id: Option<String>
    ) -> Result<u64, Box<dyn std::error::Error>> {
        let result = self
            .client
            .get_with_params::<Balance, GetBalanceRequestParams>(
                "/balance",
                params,
                self.process_custom_header(for_user_id).as_ref()
            )
            .await?;
        Ok(result.balance)
    }
}
