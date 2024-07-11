use reqwest::header::HeaderMap;

use crate::{client::XenditClient, common::ListResponse};

use super::model::{Customer, CustomerRequestBody, PatchCustomer};

pub struct CustomerClient<'a> {
    client: &'a XenditClient,
}
impl<'a> CustomerClient<'a> {
    pub fn new(client: &'a XenditClient) -> Self {
        CustomerClient { client }
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
    pub async fn create(
        &self,
        body: CustomerRequestBody,
        idempotency_key: Option<String>,
        for_user_id: Option<String>,
    ) -> Result<Customer, Box<dyn std::error::Error>> {
        let res = self
            .client
            .post::<Customer, CustomerRequestBody>(
                "/customers",
                &body,
                self.process_custom_header(for_user_id, idempotency_key)
                    .as_ref(),
            )
            .await?;
        Ok(res)
    }
    pub async fn get(
        &self,
        id: String,
        for_user_id: Option<String>,
    ) -> Result<Customer, Box<dyn std::error::Error>> {
        let res = self
            .client
            .get::<Customer>(
                &format!("/customers/{}", id),
                self.process_custom_header(for_user_id, None).as_ref(),
            )
            .await?;
        Ok(res)
    }
    pub async fn get_by_reference_id(
        &self,
        reference_id: String,
        for_user_id: Option<String>,
    ) -> Result<ListResponse<Customer>, Box<dyn std::error::Error>> {
        let res = self
            .client
            .get::<ListResponse<Customer>>(
                &format!("/customers?reference_id={}", reference_id),
                self.process_custom_header(for_user_id, None).as_ref(),
            )
            .await?;
        Ok(res)
    }
    pub async fn update(
        &self,
        id: String,
        body: PatchCustomer,
        for_user_id: Option<String>,
    ) -> Result<Customer, Box<dyn std::error::Error>> {
        let res = self
            .client
            .patch::<Customer, PatchCustomer>(
                &format!("/customers/{}", id),
                &body,
                self.process_custom_header(for_user_id, None).as_ref(),
            )
            .await?;
        Ok(res)
    }
}
