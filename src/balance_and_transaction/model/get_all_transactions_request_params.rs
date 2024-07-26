use serde::Serialize;

use crate::common::{Currency, DateRangeFilter};

use super::{ChannelCategories, TransactionStatuses, TransactionTypes};

#[derive(Serialize, Clone)]
pub struct GetAllTransactionsRequestParams {
    types: Option<Vec<TransactionTypes>>,
    statuses: Option<Vec<TransactionStatuses>>,
    channel_categories: Option<Vec<ChannelCategories>>,
    reference_id: Option<String>,
    product_id: Option<String>,
    account_identifier: Option<String>,
    currency: Option<Currency>,
    amount: Option<String>,
    created: Option<DateRangeFilter>,
    updated: Option<DateRangeFilter>,
    limit: Option<u32>,
    after_id: Option<String>,
    before_id: Option<String>,
}
impl GetAllTransactionsRequestParams {
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
    pub fn get_types(&self) -> Option<&Vec<TransactionTypes>> {
        self.types.as_ref()
    }
    pub fn set_types(&mut self, types: Vec<TransactionTypes>) -> &mut Self {
        self.types = Some(types);
        self
    }
    pub fn get_statuses(&self) -> Option<&Vec<TransactionStatuses>> {
        self.statuses.as_ref()
    }
    pub fn set_statuses(&mut self, statuses: Vec<TransactionStatuses>) -> &mut Self {
        self.statuses = Some(statuses);
        self
    }
    pub fn get_channel_categories(&self) -> Option<&Vec<ChannelCategories>> {
        self.channel_categories.as_ref()
    }
    pub fn set_channel_categories(
        &mut self,
        channel_categories: Vec<ChannelCategories>,
    ) -> &mut Self {
        self.channel_categories = Some(channel_categories);
        self
    }
    pub fn get_reference_id(&self) -> Option<&String> {
        self.reference_id.as_ref()
    }
    pub fn set_reference_id(&mut self, reference_id: String) -> &mut Self {
        self.reference_id = Some(reference_id);
        self
    }
    pub fn get_product_id(&self) -> Option<&String> {
        self.product_id.as_ref()
    }
    pub fn set_product_id(&mut self, product_id: String) -> &mut Self {
        self.product_id = Some(product_id);
        self
    }
    pub fn get_account_identifier(&self) -> Option<&String> {
        self.account_identifier.as_ref()
    }
    pub fn set_account_identifier(&mut self, account_identifier: String) -> &mut Self {
        self.account_identifier = Some(account_identifier);
        self
    }
    pub fn get_currency(&self) -> Option<&Currency> {
        self.currency.as_ref()
    }
    pub fn set_currency(&mut self, currency: Currency) -> &mut Self {
        self.currency = Some(currency);
        self
    }
    pub fn get_amount(&self) -> Option<&String> {
        self.amount.as_ref()
    }
    pub fn set_amount(&mut self, amount: String) -> &mut Self {
        self.amount = Some(amount);
        self
    }
    pub fn get_created(&self) -> Option<&DateRangeFilter> {
        self.created.as_ref()
    }
    pub fn set_created(&mut self, created: DateRangeFilter) -> &mut Self {
        self.created = Some(created);
        self
    }
    pub fn get_updated(&self) -> Option<&DateRangeFilter> {
        self.updated.as_ref()
    }
    pub fn set_updated(&mut self, updated: DateRangeFilter) -> &mut Self {
        self.updated = Some(updated);
        self
    }
    pub fn get_limit(&self) -> Option<&u32> {
        self.limit.as_ref()
    }
    pub fn set_limit(&mut self, limit: u32) -> &mut Self {
        self.limit = Some(limit);
        self
    }
    pub fn get_after_id(&self) -> Option<&String> {
        self.after_id.as_ref()
    }
    pub fn set_after_id(&mut self, after_id: String) -> &mut Self {
        self.after_id = Some(after_id);
        self
    }
    pub fn get_before_id(&self) -> Option<&String> {
        self.before_id.as_ref()
    }
    pub fn set_before_id(&mut self, before_id: String) -> &mut Self {
        self.before_id = Some(before_id);
        self
    }
    pub fn build(&self) -> Self {
        self.clone()
    }
}