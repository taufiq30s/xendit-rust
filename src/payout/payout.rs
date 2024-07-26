use reqwest::header::HeaderMap;

use crate::{client::XenditClient, common::ListResponse};

use super::{
    Channel, CreatePayoutRequest, GetPayoutChannelsRequestParams, GetPayouts200ResponseDataInner,
    GetPayoutsRequestParams,
};

pub struct PayoutClient<'a> {
    client: &'a XenditClient,
}

impl<'a> PayoutClient<'a> {
    pub fn new(client: &'a XenditClient) -> Self {
        Self { client }
    }

    fn process_custom_header(
        &self,
        for_user_id: Option<String>,
        idempotency_key: Option<String>,
    ) -> Option<HeaderMap> {
        if for_user_id.is_none() && idempotency_key.is_none() {
            return None;
        }
        let mut headers = HeaderMap::new();
        if let Some(idempotency_key) = idempotency_key {
            headers.insert("idempotency-key", idempotency_key.parse().unwrap());
        }
        if let Some(for_user_id) = for_user_id {
            headers.insert("for-user-id", for_user_id.parse().unwrap());
        }
        Some(headers)
    }

    pub async fn create_payout(
        &self,
        payout_request: CreatePayoutRequest,
        idempotency_key: String,
        for_user_id: Option<String>,
    ) -> Result<GetPayouts200ResponseDataInner, Box<dyn std::error::Error>> {
        let headers = self.process_custom_header(for_user_id, Some(idempotency_key));
        self.client
            .post::<GetPayouts200ResponseDataInner, CreatePayoutRequest>(
                "/v2/payouts",
                &payout_request,
                headers.as_ref(),
            )
            .await
    }

    pub async fn get_payout_by_id(
        &self,
        payout_id: String,
        for_user_id: Option<String>,
    ) -> Result<GetPayouts200ResponseDataInner, Box<dyn std::error::Error>> {
        let headers = self.process_custom_header(for_user_id, None);
        self.client
            .get::<GetPayouts200ResponseDataInner>(
                format!("/v2/payouts/{}", payout_id).as_str(),
                headers.as_ref(),
            )
            .await
    }

    pub async fn get_payout_channels(
        &self,
        params: GetPayoutChannelsRequestParams,
        for_user_id: Option<String>,
    ) -> Result<Vec<Channel>, Box<dyn std::error::Error>> {
        let headers = self.process_custom_header(for_user_id, None);
        self.client
            .get_with_params::<Vec<Channel>, GetPayoutChannelsRequestParams>(
                "/payouts_channels",
                params,
                headers.as_ref(),
            )
            .await
    }

    pub async fn get_payouts(
        &self,
        params: GetPayoutsRequestParams,
        for_user_id: Option<String>,
    ) -> Result<ListResponse<GetPayouts200ResponseDataInner>, Box<dyn std::error::Error>> {
        let headers = self.process_custom_header(for_user_id, None);
        self.client
            .get_with_params::<ListResponse<GetPayouts200ResponseDataInner>, GetPayoutsRequestParams>(
                "/v2/payouts", params, headers.as_ref()
            ).await
    }

    pub async fn cancel_payout(
        &self,
        id: String,
        for_user_id: Option<String>,
    ) -> Result<GetPayouts200ResponseDataInner, Box<dyn std::error::Error>> {
        let headers = self.process_custom_header(for_user_id, None);
        self.client
            .post::<GetPayouts200ResponseDataInner, ()>(
                format!("/v2/payouts/{}/cancel", id).as_str(),
                &(),
                headers.as_ref(),
            )
            .await
    }
}
