use serde::Serialize;

use crate::payment_method::PaymentStatus;

use super::{payment_type::PaymentType, reusability::Reusability};

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
    pub fn get_id(&self) -> Option<&Vec<String>> {
        self.id.as_ref()
    }
    pub fn set_id(&mut self, id: Vec<String>) -> &mut Self {
        self.id = Some(id);
        self
    }
    pub fn get_type(&self) -> Option<&Vec<PaymentType>> {
        self.r#type.as_ref()
    }
    pub fn set_type(&mut self, r#type: Vec<PaymentType>) -> &mut Self {
        self.r#type = Some(r#type);
        self
    }
    pub fn get_status(&self) -> Option<&Vec<PaymentStatus>> {
        self.status.as_ref()
    }
    pub fn set_status(&mut self, status: Vec<PaymentStatus>) -> &mut Self {
        self.status = Some(status);
        self
    }
    pub fn get_reusability(&self) -> Option<&Reusability> {
        self.reusability.as_ref()
    }
    pub fn set_reusability(&mut self, reusability: Reusability) -> &mut Self {
        self.reusability = Some(reusability);
        self
    }
    pub fn get_customer_id(&self) -> Option<&String> {
        self.customer_id.as_ref()
    }
    pub fn set_customer_id(&mut self, customer_id: String) -> &mut Self {
        self.customer_id = Some(customer_id);
        self
    }
    pub fn get_reference_id(&self) -> Option<&String> {
        self.reference_id.as_ref()
    }
    pub fn set_reference_id(&mut self, reference_id: String) -> &mut Self {
        self.reference_id = Some(reference_id);
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