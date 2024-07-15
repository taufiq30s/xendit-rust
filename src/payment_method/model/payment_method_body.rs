use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::payment_method::CardParameter;

use super::{
    country::PaymentCountry, ewallet::EWallet, payment_type::PaymentType, qr_code::QRCodeParameter,
    reusability::Reusability, BillingInformation, DirectDebitParameter, OverTheCounter,
    VirtualAccountParameter,
};

#[skip_serializing_none]
#[derive(Serialize, Clone)]
pub struct PaymentMethodBody {
    pub r#type: PaymentType,
    pub country: Option<PaymentCountry>,
    pub reusability: Reusability,
    pub customer_id: Option<String>,
    pub reference_id: Option<String>,
    pub description: Option<String>,
    pub metadata: Option<String>,
    pub card: Option<CardParameter>,
    pub direct_debit: Option<DirectDebitParameter>,
    pub ewallet: Option<EWallet>,
    pub over_the_counter: Option<OverTheCounter>,
    pub virtual_account: Option<VirtualAccountParameter>,
    pub qrcode: Option<QRCodeParameter>,
    pub billing_information: Option<BillingInformation>,
}
impl PaymentMethodBody {
    pub fn new(r#type: PaymentType, reusability: Reusability) -> Self {
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
    pub fn get_type(&self) -> &PaymentType {
        &self.r#type
    }
    pub fn set_type(&mut self, r#type: PaymentType) -> &mut Self {
        self.r#type = r#type;
        self
    }
    pub fn get_reusability(&self) -> &Reusability {
        &self.reusability
    }
    pub fn set_reusability(&mut self, reusability: Reusability) -> &mut Self {
        self.reusability = reusability;
        self
    }
    pub fn get_country(&self) -> Option<&PaymentCountry> {
        self.country.as_ref()
    }
    pub fn set_country(&mut self, country: PaymentCountry) -> &mut Self {
        self.country = Some(country);
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
    pub fn get_description(&self) -> Option<&String> {
        self.description.as_ref()
    }
    pub fn set_description(&mut self, description: String) -> &mut Self {
        self.description = Some(description);
        self
    }
    pub fn get_metadata(&self) -> Option<&String> {
        self.metadata.as_ref()
    }
    pub fn set_metadata(&mut self, metadata: String) -> &mut Self {
        self.metadata = Some(metadata);
        self
    }
    pub fn get_card(&self) -> Option<&CardParameter> {
        self.card.as_ref()
    }
    pub fn set_card(&mut self, card: CardParameter) -> &mut Self {
        self.card = Some(card);
        self
    }
    pub fn get_direct_debit(&self) -> Option<&DirectDebitParameter> {
        self.direct_debit.as_ref()
    }
    pub fn set_direct_debit(&mut self, direct_debit: DirectDebitParameter) -> &mut Self {
        self.direct_debit = Some(direct_debit);
        self
    }
    pub fn get_ewallet(&self) -> Option<&EWallet> {
        self.ewallet.as_ref()
    }
    pub fn set_ewallet(&mut self, ewallet: EWallet) -> &mut Self {
        self.ewallet = Some(ewallet);
        self
    }
    pub fn get_over_the_counter(&self) -> Option<&OverTheCounter> {
        self.over_the_counter.as_ref()
    }
    pub fn set_over_the_counter(&mut self, over_the_counter: OverTheCounter) -> &mut Self {
        self.over_the_counter = Some(over_the_counter);
        self
    }
    pub fn get_virtual_account(&self) -> Option<&VirtualAccountParameter> {
        self.virtual_account.as_ref()
    }
    pub fn set_virtual_account(&mut self, virtual_account: VirtualAccountParameter) -> &mut Self {
        self.virtual_account = Some(virtual_account);
        self
    }
    pub fn get_qrcode(&self) -> Option<&QRCodeParameter> {
        self.qrcode.as_ref()
    }
    pub fn set_qrcode(&mut self, qrcode: QRCodeParameter) -> &mut Self {
        self.qrcode = Some(qrcode);
        self
    }
    pub fn get_billing_information(&self) -> Option<&BillingInformation> {
        self.billing_information.as_ref()
    }
    pub fn set_billing_information(
        &mut self,
        billing_information: BillingInformation,
    ) -> &mut Self {
        self.billing_information = Some(billing_information);
        self
    }
    pub fn build(&self) -> Self {
        self.clone()
    }
}
