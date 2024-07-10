// Transaction

use chrono::{DateTime, Utc};
use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};

use crate::{
    client::XenditClient,
    common::{
        currency::Currency, date_filter::DateFilter, transaction_channel::ChannelCategories,
        transaction_status::TransactionStatus, transaction_type::TransactionType,
    },
};

#[derive(Serialize, Debug)]
pub struct TransactionListParams {
    types: Option<Vec<String>>,
    statuses: Option<Vec<String>>,
    channel_categories: Option<Vec<String>>,
    reference_id: Option<String>,
    product_id: Option<String>,
    account_identifier: Option<String>,
    currency: Option<String>,
    amount: Option<String>,
    created: Option<DateFilter>,
    updated: Option<DateFilter>,
    limit: Option<u32>,
    after_id: Option<String>,
    before_id: Option<String>,
}
impl TransactionListParams {
    pub fn new() -> Self {
        Self {
            types: None,
            statuses: None,
            channel_categories: None,
            reference_id: None,
            product_id: None,
            account_identifier: None,
            currency: None,
            amount: None,
            created: None,
            updated: None,
            limit: Some(10),
            after_id: None,
            before_id: None,
        }
    }
    pub fn set_types(&mut self, types: Vec<TransactionType>) -> &mut Self {
        self.types = Some(types.iter().map(|v| v.to_string()).collect::<Vec<String>>());
        self
    }
    pub fn set_statuses(&mut self, statuses: Vec<TransactionStatus>) -> &mut Self {
        self.statuses = Some(
            statuses
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>(),
        );
        self
    }
    pub fn set_channel_categories(
        &mut self,
        channel_categories: Vec<ChannelCategories>,
    ) -> &mut Self {
        self.channel_categories = Some(
            channel_categories
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<String>>(),
        );
        self
    }
    pub fn set_reference_id(&mut self, reference_id: String) -> &mut Self {
        self.reference_id = Some(reference_id);
        self
    }
    pub fn set_product_id(&mut self, product_id: String) -> &mut Self {
        self.product_id = Some(product_id);
        self
    }
    pub fn set_account_identifier(&mut self, account_identifier: String) -> &mut Self {
        self.account_identifier = Some(account_identifier);
        self
    }
    pub fn set_currency(&mut self, currency: Currency) -> &mut Self {
        self.currency = Some(currency.to_string());
        self
    }
    pub fn set_amount(&mut self, amount: String) -> &mut Self {
        self.amount = Some(amount);
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
    pub fn build(&self) -> TransactionListParams {
        TransactionListParams {
            types: self.types.clone(),
            statuses: self.statuses.clone(),
            channel_categories: self.channel_categories.clone(),
            reference_id: self.reference_id.clone(),
            product_id: self.product_id.clone(),
            account_identifier: self.account_identifier.clone(),
            currency: self.currency.clone(),
            amount: self.amount.clone(),
            created: self.created.clone(),
            updated: self.updated.clone(),
            limit: self.limit.clone(),
            after_id: self.after_id.clone(),
            before_id: self.before_id.clone(),
        }
    }
}

#[derive(Deserialize, PartialEq, Debug)]
#[allow(dead_code)]
pub struct TransactionFee {
    pub xendit_fee: u64,
    pub value_added_tax: u64,
    pub xendit_withholding_tax: u64,
    pub third_party_withholding_tax: u64,
    pub status: String,
}

#[derive(Deserialize, PartialEq, Debug)]
#[allow(dead_code)]
pub struct TransactionObject {
    pub id: String,
    pub product_id: String,
    pub r#type: String,
    pub channel_code: Option<String>,
    pub reference_id: Option<String>,
    pub account_identifier: Option<String>,
    pub currency: Option<String>,
    pub amount: u64,
    pub net_amount: u64,
    pub cashflow: String,
    pub status: String,
    pub channel_category: String,
    pub business_id: String,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub fee: TransactionFee,
    pub settlement_status: Option<String>,
    pub estimated_settlement_time: Option<DateTime<Utc>>,
}

#[derive(Deserialize, PartialEq, Debug)]
#[allow(dead_code)]
pub struct TransactionLinks {
    pub href: String,
    pub method: String,
    pub rel: String,
}

#[derive(Deserialize, PartialEq, Debug)]
#[allow(dead_code)]
pub struct TransactionListObject {
    pub data: Vec<TransactionObject>,
    pub has_more: bool,
    pub links: Option<Vec<TransactionLinks>>,
}

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
    pub async fn list(
        &self,
        params: TransactionListParams,
        for_user_id: Option<String>,
    ) -> Result<TransactionListObject, Box<dyn std::error::Error>> {
        let result = self
            .client
            .get_with_params::<TransactionListObject, _>(
                "/transactions",
                params,
                self.process_custom_header(for_user_id).as_ref(),
            )
            .await?;
        Ok(result)
    }
    pub async fn get(
        &self,
        id: String,
        for_user_id: Option<String>,
    ) -> Result<TransactionObject, Box<dyn std::error::Error>> {
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
