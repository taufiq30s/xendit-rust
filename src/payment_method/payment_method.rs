use crate::{client::XenditClient, common::ListResponse};
use reqwest::header::HeaderMap;
use std::collections::HashMap;

use super::{
    GetPaymentByPaymentMethodIdParams, GetPaymentMethodsParams, PaymentMethod, PaymentMethodBody,
    PaymentMethodExpireParams, PaymentMethodUpdateBody, SimulateResponse,
};

pub struct PaymentMethodClient<'a> {
    client: &'a XenditClient,
}
impl<'a> PaymentMethodClient<'a> {
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
    pub async fn create_payment_method(
        &self,
        body: PaymentMethodBody,
        for_user_id: Option<String>,
    ) -> Result<PaymentMethod, Box<dyn std::error::Error>> {
        let res = self
            .client
            .post::<PaymentMethod, PaymentMethodBody>(
                "/v2/payment_methods",
                &body,
                self.process_custom_header(for_user_id).as_ref(),
            )
            .await?;
        Ok(res)
    }
    pub async fn get_payment_method_by_id(
        &self,
        id: &str,
        for_user_id: Option<String>,
    ) -> Result<PaymentMethod, Box<dyn std::error::Error>> {
        let res = self
            .client
            .get::<PaymentMethod>(
                &format!("v2/payment_methods/{}", id),
                self.process_custom_header(for_user_id).as_ref(),
            )
            .await?;
        Ok(res)
    }
    pub async fn get_payments_by_payment_method_id(
        &self,
        id: String,
        params: Option<GetPaymentByPaymentMethodIdParams>,
        for_user_id: Option<String>,
    ) -> Result<ListResponse<PaymentMethod>, Box<dyn std::error::Error>> {
        let res = match params {
            Some(p) => {
                self.client
                    .get_with_params::<ListResponse<PaymentMethod>, _>(
                        &format!("v2/payment_methods/{}/payments", id),
                        p,
                        self.process_custom_header(for_user_id).as_ref(),
                    )
                    .await?
            }
            None => {
                self.client
                    .get::<ListResponse<PaymentMethod>>(
                        &format!("v2/payment_methods/{}/payments", id),
                        self.process_custom_header(for_user_id).as_ref(),
                    )
                    .await?
            }
        };
        Ok(res)
    }
    pub async fn patch_payment_method(
        &self,
        id: &str,
        body: PaymentMethodUpdateBody,
        for_user_id: Option<String>,
    ) -> Result<PaymentMethod, Box<dyn std::error::Error>> {
        let res = self
            .client
            .patch::<PaymentMethod, PaymentMethodUpdateBody>(
                &format!("v2/payment_methods/{}", id),
                &body,
                self.process_custom_header(for_user_id).as_ref(),
            )
            .await?;
        Ok(res)
    }
    pub async fn get_all_payment_methods(
        &self,
        params: Option<GetPaymentMethodsParams>,
        for_user_id: Option<String>,
    ) -> Result<ListResponse<PaymentMethod>, Box<dyn std::error::Error>> {
        let res = match params {
            Some(p) => {
                self.client
                    .get_with_params::<ListResponse<PaymentMethod>, _>(
                        "/v2/payment_methods",
                        p,
                        self.process_custom_header(for_user_id).as_ref(),
                    )
                    .await?
            }
            None => {
                self.client
                    .get::<ListResponse<PaymentMethod>>(
                        "/v2/payment_methods",
                        self.process_custom_header(for_user_id).as_ref(),
                    )
                    .await?
            }
        };
        Ok(res)
    }
    pub async fn expire_payment_method(
        &self,
        id: &str,
        params: Option<PaymentMethodExpireParams>,
        for_user_id: Option<String>,
    ) -> Result<PaymentMethod, Box<dyn std::error::Error>> {
        let res = self
            .client
            .post_with_params::<PaymentMethod, (), _>(
                &format!("v2/payment_methods/{}/expire", id),
                &(),
                params,
                self.process_custom_header(for_user_id).as_ref(),
            )
            .await?;
        Ok(res)
    }
    pub async fn auth_payment_method(
        &self,
        id: &str,
        auth_code: &str,
        for_user_id: Option<String>,
    ) -> Result<PaymentMethod, Box<dyn std::error::Error>> {
        let body: HashMap<String, String> = [("auth_code".to_string(), auth_code.to_string())]
            .into_iter()
            .collect();
        let res = self
            .client
            .post::<PaymentMethod, HashMap<String, String>>(
                &format!("v2/payment_methods/{}/auth", id),
                &body,
                self.process_custom_header(for_user_id).as_ref(),
            )
            .await?;
        Ok(res)
    }
    pub async fn simulate_payment(
        &self,
        id: &str,
        amount: f64,
    ) -> Result<SimulateResponse, Box<dyn std::error::Error>> {
        let body: HashMap<String, f64> = [("amount".to_string(), amount)].into_iter().collect();
        let res = self
            .client
            .post::<SimulateResponse, HashMap<String, f64>>(
                &format!("/v2/payment_methods/{}/payments/simulate", id),
                &body,
                None,
            )
            .await?;
        Ok(res)
    }
}
