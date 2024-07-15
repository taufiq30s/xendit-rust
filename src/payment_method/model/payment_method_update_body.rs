use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::payment_method::{
    OverTheCounterParameterUpdate, PaymentStatus, VirtualAccountParameterUpdate,
};

use super::reusability::Reusability;

#[skip_serializing_none]
#[derive(Serialize, Clone)]
pub struct PaymentMethodUpdateBody {
    description: Option<String>,
    reference_id: Option<String>,
    reusability: Option<Reusability>,
    status: Option<PaymentStatus>,
    over_the_counter: Option<OverTheCounterParameterUpdate>,
    virtual_account: Option<VirtualAccountParameterUpdate>,
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
    pub fn get_description(&self) -> Option<&String> {
        self.description.as_ref()
    }
    pub fn set_description(&mut self, description: String) -> &mut Self {
        self.description = Some(description);
        self
    }
    pub fn get_reference_id(&self) -> Option<&String> {
        self.reference_id.as_ref()
    }
    pub fn set_reference_id(&mut self, reference_id: String) -> &mut Self {
        self.reference_id = Some(reference_id);
        self
    }
    pub fn get_reusability(&self) -> Option<&Reusability> {
        self.reusability.as_ref()
    }
    pub fn set_reusability(&mut self, reusability: Reusability) -> &mut Self {
        self.reusability = Some(reusability);
        self
    }
    pub fn get_status(&self) -> Option<&PaymentStatus> {
        self.status.as_ref()
    }
    pub fn set_status(&mut self, status: PaymentStatus) -> &mut Self {
        self.status = Some(status);
        self
    }
    pub fn get_over_the_counter(&self) -> Option<&OverTheCounterParameterUpdate> {
        self.over_the_counter.as_ref()
    }
    pub fn set_over_the_counter(
        &mut self,
        over_the_counter: OverTheCounterParameterUpdate,
    ) -> &mut Self {
        self.over_the_counter = Some(over_the_counter);
        self
    }
    pub fn get_virtual_account(&self) -> Option<&VirtualAccountParameterUpdate> {
        self.virtual_account.as_ref()
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
