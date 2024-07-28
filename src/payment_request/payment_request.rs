use reqwest::header::HeaderMap;

use crate::{client::XenditClient, common::ListResponse};

use super::{Capture, PaymentRequest, PaymentRequestParameters, PaymentSimulation};

pub struct PaymentRequestClient<'a> {
    client: &'a XenditClient,
}
impl<'a> PaymentRequestClient<'a> {
    pub fn new(client: &'a XenditClient) -> Self {
        PaymentRequestClient { client }
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

    pub async fn create_payment_request(
        &self,
        body: PaymentRequestParameters,
        idempotency_key: Option<String>,
        for_user_id: Option<String>,
    ) -> Result<PaymentRequest, Box<dyn std::error::Error>> {
        println!("Create Payment Request");
        let response = self
            .client
            .post::<PaymentRequest, PaymentRequestParameters>(
                "/payment_requests",
                &body,
                self.process_custom_header(for_user_id, idempotency_key)
                    .as_ref(),
            )
            .await?;
        Ok(response)
    }

    pub async fn get_payment_request_by_id(
        &self,
        payment_request_id: String,
        for_user_id: Option<String>,
    ) -> Result<PaymentRequest, Box<dyn std::error::Error>> {
        let response = self
            .client
            .get::<PaymentRequest>(
                &format!("/payment_requests/{}", payment_request_id),
                self.process_custom_header(for_user_id, None).as_ref(),
            )
            .await?;
        Ok(response)
    }

	pub async fn get_payment_request_capture(
        &self,
        payment_request_id: String,
        for_user_id: Option<String>,
    ) -> Result<ListResponse<Capture>, Box<dyn std::error::Error>> {
        println!("Get Capture");
        let response = self
            .client
            .get::<ListResponse<Capture>>(
                &format!("/payment_requests/{}/captures", payment_request_id),
                self.process_custom_header(for_user_id, None).as_ref(),
            )
            .await?;
        Ok(response)
    }

	pub async fn get_all_payment_requests(
        &self,
        for_user_id: Option<String>,
    ) -> Result<ListResponse<PaymentRequest>, Box<dyn std::error::Error>> {
        let response = self
            .client
            .get::<ListResponse<PaymentRequest>>(
                "/payment_requests",
                self.process_custom_header(for_user_id, None).as_ref(),
            )
            .await?;
        Ok(response)
    }

	pub async fn capture_payment_request(
        &self,
        payment_request_id: String,
        for_user_id: Option<String>,
    ) -> Result<Capture, Box<dyn std::error::Error>> {
        let response = self
            .client
            .post::<Capture, ()>(
                &format!("/payment_requests/{}/captures", payment_request_id),
                &(),
                self.process_custom_header(for_user_id, None).as_ref(),
            )
            .await?;
        Ok(response)
    }

	pub async fn authorize_payment_request(
        &self,
        payment_request_id: String,
        for_user_id: Option<String>,
    ) -> Result<PaymentRequest, Box<dyn std::error::Error>> {
        let response = self
            .client
            .post::<PaymentRequest, ()>(
                &format!("/payment_requests/{}/auth", payment_request_id),
                &(),
                self.process_custom_header(for_user_id, None).as_ref(),
            )
            .await?;
        Ok(response)
    }

	pub async fn resend_payment_request_auth(
        &self,
        payment_request_id: String,
        for_user_id: Option<String>,
    ) -> Result<PaymentRequest, Box<dyn std::error::Error>> {
        let response = self
            .client
            .post::<PaymentRequest, ()>(
                &format!("/payment_requests/{}/auth/resend", payment_request_id),
                &(),
                self.process_custom_header(for_user_id, None).as_ref(),
            )
            .await?;
        Ok(response)
    }

    pub async fn simulate_payment_request(
        &self,
        payment_request_id: String,
        for_user_id: Option<String>,
    ) -> Result<PaymentSimulation, Box<dyn std::error::Error>> {
        let response = self
            .client
            .post::<PaymentSimulation, ()>(
                &format!("/payment_requests/{}/payments/simulate", payment_request_id),
                &(),
                self.process_custom_header(for_user_id, None).as_ref(),
            )
            .await?;
        Ok(response)
    }
}
