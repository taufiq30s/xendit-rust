use reqwest::header::HeaderMap;

use crate::{client::XenditClient, common::ListResponse};

use super::{CreateRefund, Refund};

pub struct RefundClient<'a> {
    client: &'a XenditClient,
}

impl<'a> RefundClient<'a> {
    pub fn new(client: &'a XenditClient) -> Self {
        Self { client }
    }
    fn process_custom_header(
        &self,
        for_user_id: Option<String>,
        idempotency_key: Option<String>,
    ) -> Option<HeaderMap> {
        if for_user_id.is_none() || idempotency_key.is_none() {
            return None;
        }
        let mut headers = HeaderMap::new();
        if let Some(idempotency_key) = idempotency_key {
            headers.insert("x-idempotency-key", idempotency_key.parse().unwrap());
        }
        if let Some(for_user_id) = for_user_id {
            headers.insert("for-user-id", for_user_id.parse().unwrap());
        }
        Some(headers)
    }

    pub async fn create_refund(
        &self,
        body: CreateRefund,
        for_user_id: Option<String>,
        idempotency_key: Option<String>,
    ) -> Result<Refund, Box<dyn std::error::Error>> {
        let response = self
            .client
            .post::<Refund, CreateRefund>(
                "/refunds",
                &body,
                self.process_custom_header(for_user_id, idempotency_key)
                    .as_ref(),
            )
            .await?;
        Ok(response)
    }
    pub async fn get_refund(
        &self,
        id: String,
        for_user_id: Option<String>,
    ) -> Result<Refund, Box<dyn std::error::Error>> {
        let response = self
            .client
            .get::<Refund>(
                &format!("/refunds/{}", id),
                self.process_custom_header(for_user_id, None).as_ref(),
            )
            .await?;
        Ok(response)
    }
    pub async fn get_all_refund(
        &self,
        for_user_id: Option<String>,
    ) -> Result<ListResponse<Refund>, Box<dyn std::error::Error>> {
        let response = self
            .client
            .get::<ListResponse<Refund>>(
                "/refunds",
                self.process_custom_header(for_user_id, None).as_ref(),
            )
            .await?;
        Ok(response)
    }
    pub async fn cancel_refund(
        &self,
        id: String,
        for_user_id: Option<String>,
    ) -> Result<Refund, Box<dyn std::error::Error>> {
        let response = self
            .client
            .post::<Refund, ()>(
                &format!("/refunds/{}/cancel", id),
                &(),
                self.process_custom_header(for_user_id, None).as_ref(),
            )
            .await?;
        Ok(response)
    }
}
