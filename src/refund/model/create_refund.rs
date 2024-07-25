use std::collections::HashMap;

use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::common::Currency;

use super::RefundReason;

#[skip_serializing_none]
#[derive(Serialize, Clone)]
pub struct CreateRefund {
    payment_request_id: Option<String>,
    invoice_id: Option<String>,
    reference_id: Option<String>,
    amount: Option<f64>,
    currency: Option<Currency>,
    reason: RefundReason,
    metadata: Option<HashMap<String, String>>,
}
impl CreateRefund {
    pub fn new(reason: RefundReason) -> Self {
        Self {
            payment_request_id: None,
            invoice_id: None,
            reference_id: None,
            amount: None,
            currency: None,
            reason,
            metadata: None,
        }
    }
    pub fn get_payment_request_id(&self) -> Option<&String> {
        self.payment_request_id.as_ref()
    }
    pub fn set_payment_request_id(&mut self, payment_request_id: &str) -> &mut Self {
        self.payment_request_id = Some(payment_request_id.to_string());
        self
    }
    pub fn get_invoice_id(&self) -> Option<&String> {
        self.invoice_id.as_ref()
    }
    pub fn set_invoice_id(&mut self, invoice_id: &str) -> &mut Self {
        self.invoice_id = Some(invoice_id.to_string());
        self
    }
    pub fn get_reference_id(&self) -> Option<&String> {
        self.reference_id.as_ref()
    }
    pub fn set_reference_id(&mut self, reference_id: &str) -> &mut Self {
        self.reference_id = Some(reference_id.to_string());
        self
    }
    pub fn get_amount(&self) -> Option<&f64> {
        self.amount.as_ref()
    }
    pub fn set_amount(&mut self, amount: f64) -> &mut Self {
        self.amount = Some(amount);
        self
    }
    pub fn get_currency(&self) -> Option<&Currency> {
        self.currency.as_ref()
    }
    pub fn set_currency(&mut self, currency: Currency) -> &mut Self {
        self.currency = Some(currency);
        self
    }
    pub fn get_reason(&self) -> &RefundReason {
        &self.reason
    }
    pub fn set_reason(&mut self, reason: RefundReason) -> &mut Self {
        self.reason = reason;
        self
    }
    pub fn get_metadata(&self) -> Option<&HashMap<String, String>> {
        self.metadata.as_ref()
    }
    pub fn set_metadata(&mut self, metadata: HashMap<String, String>) -> &mut Self {
        self.metadata = Some(metadata);
        self
    }
    pub fn build(&mut self) -> CreateRefund {
        self.clone()
    }
}
