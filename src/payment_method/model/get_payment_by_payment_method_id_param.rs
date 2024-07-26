use serde::Serialize;

use crate::{common::DateRangeFilter, payment_method::PaymentStatus};

#[derive(Serialize, Clone)]
pub struct GetPaymentByPaymentMethodIdParams {
    payment_request_id: Option<String>,
    reference_id: Option<String>,
    status: Option<PaymentStatus>,
    limit: Option<i64>,
    after_id: Option<String>,
    before_id: Option<String>,
    created: Option<DateRangeFilter>,
    updated: Option<DateRangeFilter>,
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
    pub fn get_payment_method_id(&self) -> Option<&String> {
        self.payment_request_id.as_ref()
    }
    pub fn set_payment_method_id(&mut self, payment_method_id: String) -> &mut Self {
        self.payment_request_id = Some(payment_method_id);
        self
    }
    pub fn get_reference_id(&self) -> Option<&String> {
        self.reference_id.as_ref()
    }
    pub fn set_reference_id(&mut self, reference_id: String) -> &mut Self {
        self.reference_id = Some(reference_id);
        self
    }
    pub fn get_status(&self) -> Option<&PaymentStatus> {
        self.status.as_ref()
    }
    pub fn set_status(&mut self, status: PaymentStatus) -> &mut Self {
        self.status = Some(status);
        self
    }
    pub fn get_limit(&self) -> Option<&i64> {
        self.limit.as_ref()
    }
    pub fn set_limit(&mut self, limit: i64) -> &mut Self {
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
    pub fn build(&self) -> Self {
        self.clone()
    }
}