// Transaction

use reqwest::header::HeaderMap;

use crate::{client::XenditClient, common::ListResponse};

use super::model::{GetAllTransactionsRequestParams, Transaction};

pub struct TransactionClient<'a> {
    client: &'a XenditClient,
}
impl<'a> TransactionClient<'a> {
    pub fn new(client: &'a XenditClient) -> Self {
        Self { client }
    }
    fn process_custom_header(&self, for_user_id: Option<String>) -> Option<HeaderMap> {
        if for_user_id.is_none() {
            return None;
        }
        let mut headers = HeaderMap::new();
        headers.insert("for-user-id", for_user_id.unwrap().parse().unwrap());
        Some(headers)
    }
    pub async fn get_all_transactions(
        &self,
        params: GetAllTransactionsRequestParams,
        for_user_id: Option<String>,
    ) -> Result<ListResponse<Transaction>, Box<dyn std::error::Error>> {
        let result = self
            .client
            .get_with_params::<ListResponse<Transaction>, GetAllTransactionsRequestParams>(
                "/transactions",
                params,
                self.process_custom_header(for_user_id).as_ref(),
            )
            .await?;
        Ok(result)
    }
    pub async fn get_transaction_by_id(
        &self,
        id: String,
        for_user_id: Option<String>,
    ) -> Result<Transaction, Box<dyn std::error::Error>> {
        let result = self
            .client
            .get(
                &format!("/transactions/{}", id),
                self.process_custom_header(for_user_id).as_ref(),
            )
            .await?;
        Ok(result)
    }
}
