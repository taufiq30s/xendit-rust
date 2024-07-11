use std::collections::HashMap;

use chrono::{DateTime, Utc};
use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::skip_serializing_none;

use crate::{client::XenditClient, common::{date_filter::DateFilter, ListResponse}, invoice::Address};

use super::payment_method_parameters::{
    self as PaymentMethodObject, OverTheCounterParameterUpdate, PaymentType, Reusability,
    VirtualAccountParameterUpdate,
};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PaymentStatus {
    Active,
    Expired,
    Inactive,
    Pending,
    RequiresAction,
    Failed,
    Succeeded,
    XenditEnumDefaultFallback,
}

#[derive(Deserialize)]
pub struct Action {
    pub action: String,
    pub method: String,
    pub url_type: String,
    pub url: String,
}

#[skip_serializing_none]
#[derive(Serialize, Clone)]
pub struct PaymentMethodBody {
    pub r#type: PaymentMethodObject::PaymentType,
    pub country: Option<PaymentMethodObject::PaymentCountry>,
    pub reusability: PaymentMethodObject::Reusability,
    pub customer_id: Option<String>,
    pub reference_id: Option<String>,
    pub description: Option<String>,
    pub metadata: Option<String>,
    pub card: Option<PaymentMethodObject::CardParameter>,
    pub direct_debit: Option<PaymentMethodObject::DirectDebitParameter>,
    pub ewallet: Option<PaymentMethodObject::EWalletParameter>,
    pub over_the_counter: Option<PaymentMethodObject::OverTheCounterParameter>,
    pub virtual_account: Option<PaymentMethodObject::VirtualAccountParameter>,
    pub qrcode: Option<PaymentMethodObject::QRCodeParameter>,
    pub billing_information: Option<Address>,
}
impl PaymentMethodBody {
    pub fn new(
        r#type: PaymentMethodObject::PaymentType,
        reusability: PaymentMethodObject::Reusability,
    ) -> Self {
        Self {
            r#type,
            country: None,
            reusability,
            customer_id: None,
            reference_id: None,
            description: None,
            metadata: None,
            card: None,
            direct_debit: None,
            ewallet: None,
            over_the_counter: None,
            virtual_account: None,
            qrcode: None,
            billing_information: None,
        }
    }
    pub fn set_country(&mut self, country: PaymentMethodObject::PaymentCountry) -> &mut Self {
        self.country = Some(country);
        self
    }
    pub fn set_customer_id(&mut self, customer_id: String) -> &mut Self {
        self.customer_id = Some(customer_id);
        self
    }
    pub fn set_reference_id(&mut self, reference_id: String) -> &mut Self {
        self.reference_id = Some(reference_id);
        self
    }
    pub fn set_description(&mut self, description: String) -> &mut Self {
        self.description = Some(description);
        self
    }
    pub fn set_metadata(&mut self, metadata: String) -> &mut Self {
        self.metadata = Some(metadata);
        self
    }
    pub fn set_card(&mut self, card: PaymentMethodObject::CardParameter) -> &mut Self {
        self.card = Some(card);
        self
    }
    pub fn set_direct_debit(
        &mut self,
        direct_debit: PaymentMethodObject::DirectDebitParameter,
    ) -> &mut Self {
        self.direct_debit = Some(direct_debit);
        self
    }
    pub fn set_ewallet(&mut self, ewallet: PaymentMethodObject::EWalletParameter) -> &mut Self {
        self.ewallet = Some(ewallet);
        self
    }
    pub fn set_over_the_counter(
        &mut self,
        over_the_counter: PaymentMethodObject::OverTheCounterParameter,
    ) -> &mut Self {
        self.over_the_counter = Some(over_the_counter);
        self
    }
    pub fn set_virtual_account(
        &mut self,
        virtual_account: PaymentMethodObject::VirtualAccountParameter,
    ) -> &mut Self {
        self.virtual_account = Some(virtual_account);
        self
    }
    pub fn set_qrcode(&mut self, qrcode: PaymentMethodObject::QRCodeParameter) -> &mut Self {
        self.qrcode = Some(qrcode);
        self
    }
    pub fn set_billing_information(&mut self, billing_information: Address) -> &mut Self {
        self.billing_information = Some(billing_information);
        self
    }
    pub fn build(&self) -> Self {
        self.clone()
    }
}

#[skip_serializing_none]
#[derive(Serialize, Clone)]
pub struct PaymentMethodUpdateBody {
    pub description: Option<String>,
    pub reference_id: Option<String>,
    pub reusability: Option<Reusability>,
    pub status: Option<PaymentStatus>,
    pub over_the_counter: Option<OverTheCounterParameterUpdate>,
    pub virtual_account: Option<VirtualAccountParameterUpdate>,
}
impl PaymentMethodUpdateBody {
    pub fn new() -> Self {
        Self {
            description: None,
            reference_id: None,
            reusability: None,
            status: None,
            over_the_counter: None,
            virtual_account: None,
        }
    }
    pub fn set_description(&mut self, description: String) -> &mut Self {
        self.description = Some(description);
        self
    }
    pub fn set_reference_id(&mut self, reference_id: String) -> &mut Self {
        self.reference_id = Some(reference_id);
        self
    }
    pub fn set_reusability(&mut self, reusability: Reusability) -> &mut Self {
        self.reusability = Some(reusability);
        self
    }
    pub fn set_status(&mut self, status: PaymentStatus) -> &mut Self {
        self.status = Some(status);
        self
    }
    pub fn set_over_the_counter(
        &mut self,
        over_the_counter: OverTheCounterParameterUpdate,
    ) -> &mut Self {
        self.over_the_counter = Some(over_the_counter);
        self
    }
    pub fn set_virtual_account(
        &mut self,
        virtual_account: VirtualAccountParameterUpdate,
    ) -> &mut Self {
        self.virtual_account = Some(virtual_account);
        self
    }
    pub fn build(&self) -> Self {
        self.clone()
    }
}

#[derive(Serialize, Clone)]
pub struct GetPaymentByPaymentMethodIdParams {
    pub payment_request_id: Option<String>,
    pub reference_id: Option<String>,
    pub status: Option<PaymentStatus>,
    pub limit: Option<i64>,
    pub after_id: Option<String>,
    pub before_id: Option<String>,
    pub created: Option<DateFilter>,
    pub updated: Option<DateFilter>,
}
impl GetPaymentByPaymentMethodIdParams {
    pub fn new(payment_method_id: &str) -> Self {
        Self {
            payment_request_id: Some(payment_method_id.to_string()),
            reference_id: None,
            status: None,
            limit: None,
            after_id: None,
            before_id: None,
            created: None,
            updated: None,
        }
    }
    pub fn set_reference_id(&mut self, reference_id: String) -> &mut Self {
        self.reference_id = Some(reference_id);
        self
    }
    pub fn set_status(&mut self, status: PaymentStatus) -> &mut Self {
        self.status = Some(status);
        self
    }
    pub fn set_limit(&mut self, limit: i64) -> &mut Self {
        self.limit = Some(limit);
        self
    }
    pub fn set_after_id(&mut self, after_id: String) -> &mut Self {
        self.after_id = Some(after_id);
        self
    }
    pub fn set_before_id(&mut self, before_id: String) -> &mut Self {
        self.before_id = Some(before_id);
        self
    }
    pub fn set_created(&mut self, created: DateFilter) -> &mut Self {
        self.created = Some(created);
        self
    }
    pub fn set_updated(&mut self, updated: DateFilter) -> &mut Self {
        self.updated = Some(updated);
        self
    }
    pub fn build(&self) -> Self {
        self.clone()
    }
}

#[derive(Serialize, Clone)]
pub struct GetPaymentMethodsParams {
    pub id: Option<Vec<String>>,
    pub r#type: Option<Vec<PaymentType>>,
    pub status: Option<Vec<PaymentStatus>>,
    pub reusability: Option<Reusability>,
    pub customer_id: Option<String>,
    pub reference_id: Option<String>,
    pub limit: Option<u32>,
    pub after_id: Option<String>,
    pub before_id: Option<String>,
}
impl GetPaymentMethodsParams {
    pub fn new() -> Self {
        Self {
            id: None,
            r#type: None,
            status: None,
            reusability: None,
            customer_id: None,
            reference_id: None,
            limit: None,
            after_id: None,
            before_id: None,
        }
    }
    pub fn set_id(&mut self, id: Vec<String>) -> &mut Self {
        self.id = Some(id);
        self
    }
    pub fn set_type(&mut self, r#type: Vec<PaymentType>) -> &mut Self {
        self.r#type = Some(r#type);
        self
    }
    pub fn set_status(&mut self, status: Vec<PaymentStatus>) -> &mut Self {
        self.status = Some(status);
        self
    }
    pub fn set_reusability(&mut self, reusability: Reusability) -> &mut Self {
        self.reusability = Some(reusability);
        self
    }
    pub fn set_customer_id(&mut self, customer_id: String) -> &mut Self {
        self.customer_id = Some(customer_id);
        self
    }
    pub fn set_reference_id(&mut self, reference_id: String) -> &mut Self {
        self.reference_id = Some(reference_id);
        self
    }
    pub fn set_limit(&mut self, limit: u32) -> &mut Self {
        self.limit = Some(limit);
        self
    }
    pub fn set_after_id(&mut self, after_id: String) -> &mut Self {
        self.after_id = Some(after_id);
        self
    }
    pub fn set_before_id(&mut self, before_id: String) -> &mut Self {
        self.before_id = Some(before_id);
        self
    }
    pub fn build(&self) -> Self {
        self.clone()
    }
}

#[derive(Serialize, Clone)]
pub struct PaymentMethodExpireParams {
    pub success_return_url: Option<String>,
    pub failure_return_url: Option<String>,
}
impl PaymentMethodExpireParams {
    pub fn new(success_return_url: String, failure_return_url: String) -> Self {
        Self {
            success_return_url: Some(success_return_url),
            failure_return_url: Some(failure_return_url),
        }
    }
}

#[derive(Deserialize)]
pub struct PaymentMethod {
    pub id: String,
    pub business_id: Option<String>,
    pub customer_id: Option<String>,
    pub customer: Option<HashMap<String, Value>>,
    pub reference_id: Option<String>,
    pub reusability: Option<PaymentMethodObject::Reusability>,
    pub country: PaymentMethodObject::PaymentCountry,
    pub status: PaymentStatus,
    pub actions: Option<Vec<Action>>,
    pub r#type: Option<PaymentMethodObject::PaymentType>,
    pub ewallet: Option<PaymentMethodObject::EWalletParameter>,
    pub direct_debit: Option<PaymentMethodObject::DirectDebitParameter>,
    pub card: Option<PaymentMethodObject::CardParameter>,
    pub over_the_counter: Option<PaymentMethodObject::OverTheCounterParameter>,
    pub virtual_account: Option<PaymentMethodObject::VirtualAccountParameter>,
    pub qrcode: Option<PaymentMethodObject::QRCodeParameter>,
    pub description: Option<String>,
    pub billing_information: Option<Address>,
    pub failure_code: Option<String>,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub metadata: Option<HashMap<String, Value>>,
}

#[derive(Deserialize, Debug)]
pub struct SimulateResponse {
    pub status: String,
    pub message: String,
}

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
        for_user_id: Option<String>
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
        for_user_id: Option<String>
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
        for_user_id: Option<String>
    ) -> Result<PaymentMethod, Box<dyn std::error::Error>> {
        let res = self
            .client
            .patch::<PaymentMethod, PaymentMethodUpdateBody>(
                &format!("v2/payment_methods/{}", id),
                &body,
                self.process_custom_header(for_user_id).as_ref()
            )
            .await?;
        Ok(res)
    }
    pub async fn get_all_payment_methods(
        &self,
        params: Option<GetPaymentMethodsParams>,
        for_user_id: Option<String>
    ) -> Result<ListResponse<PaymentMethod>, Box<dyn std::error::Error>> {
        let res = match params {
            Some(p) => {
                self.client
                    .get_with_params::<ListResponse<PaymentMethod>, _>(
                        "/v2/payment_methods", 
                        p,
                        self.process_custom_header(for_user_id).as_ref()
                    )
                    .await?
            }
            None => {
                self.client
                    .get::<ListResponse<PaymentMethod>>(
                        "/v2/payment_methods",
                        self.process_custom_header(for_user_id).as_ref()
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
        for_user_id: Option<String>
    ) -> Result<PaymentMethod, Box<dyn std::error::Error>> {
        let res = self
            .client
            .post_with_params::<PaymentMethod, (), _>(
                &format!("v2/payment_methods/{}/expire", id),
                &(),
                params,
                self.process_custom_header(for_user_id).as_ref()
            )
            .await?;
        Ok(res)
    }
    pub async fn auth_payment_method(
        &self,
        id: &str,
        auth_code: &str,
        for_user_id: Option<String>
    ) -> Result<PaymentMethod, Box<dyn std::error::Error>> {
        let body: HashMap<String, String> = [("auth_code".to_string(), auth_code.to_string())]
            .into_iter()
            .collect();
        let res = self
            .client
            .post::<PaymentMethod, HashMap<String, String>>(
                &format!("v2/payment_methods/{}/auth", id),
                &body,
                self.process_custom_header(for_user_id).as_ref()
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
                None
            )
            .await?;
        Ok(res)
    }
}
