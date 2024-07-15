use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_json::Value;

use crate::payment_method::{PaymentStatus, VirtualAccount};

use super::{
    action::Action, country::PaymentCountry, direct_debit::DirectDebit, ewallet::EWallet,
    payment_type::PaymentType, qr_code::QRCodeParameter, reusability::Reusability,
    BillingInformation, Card, OverTheCounter,
};

#[derive(Deserialize)]
pub struct PaymentMethod {
    id: String,
    business_id: Option<String>,
    customer_id: Option<String>,
    customer: Option<HashMap<String, Value>>,
    reference_id: Option<String>,
    reusability: Option<Reusability>,
    country: Option<PaymentCountry>,
    status: PaymentStatus,
    actions: Option<Vec<Action>>,
    r#type: Option<PaymentType>,
    ewallet: Option<EWallet>,
    direct_debit: Option<DirectDebit>,
    card: Option<Card>,
    over_the_counter: Option<OverTheCounter>,
    virtual_account: Option<VirtualAccount>,
    qrcode: Option<QRCodeParameter>,
    description: Option<String>,
    billing_information: Option<BillingInformation>,
    failure_code: Option<String>,
    created: DateTime<Utc>,
    updated: DateTime<Utc>,
    metadata: Option<HashMap<String, Value>>,
}
impl PaymentMethod {
    pub fn get_id(&self) -> &str {
        &self.id
    }
    pub fn get_business_id(&self) -> Option<&String> {
        self.business_id.as_ref()
    }
    pub fn get_customer_id(&self) -> Option<&String> {
        self.customer_id.as_ref()
    }
    pub fn get_customer(&self) -> Option<&HashMap<String, Value>> {
        self.customer.as_ref()
    }
    pub fn get_reference_id(&self) -> Option<&String> {
        self.reference_id.as_ref()
    }
    pub fn get_reusability(&self) -> Option<&Reusability> {
        self.reusability.as_ref()
    }
    pub fn get_country(&self) -> Option<&PaymentCountry> {
        self.country.as_ref()
    }
    pub fn get_status(&self) -> &PaymentStatus {
        &self.status
    }
    pub fn get_actions(&self) -> Option<&Vec<Action>> {
        self.actions.as_ref()
    }
    pub fn get_type(&self) -> Option<&PaymentType> {
        self.r#type.as_ref()
    }
    pub fn get_ewallet(&self) -> Option<&EWallet> {
        self.ewallet.as_ref()
    }
    pub fn get_direct_debit(&self) -> Option<&DirectDebit> {
        self.direct_debit.as_ref()
    }
    pub fn get_card(&self) -> Option<&Card> {
        self.card.as_ref()
    }
    pub fn get_over_the_counter(&self) -> Option<&OverTheCounter> {
        self.over_the_counter.as_ref()
    }
    pub fn get_virtual_account(&self) -> Option<&VirtualAccount> {
        self.virtual_account.as_ref()
    }
    pub fn get_qrcode(&self) -> Option<&QRCodeParameter> {
        self.qrcode.as_ref()
    }
    pub fn get_description(&self) -> Option<&String> {
        self.description.as_ref()
    }
    pub fn get_billing_information(&self) -> Option<&BillingInformation> {
        self.billing_information.as_ref()
    }
    pub fn get_failure_code(&self) -> Option<&String> {
        self.failure_code.as_ref()
    }
    pub fn get_created(&self) -> &DateTime<Utc> {
        &self.created
    }
    pub fn get_updated(&self) -> &DateTime<Utc> {
        &self.updated
    }
    pub fn get_metadata(&self) -> Option<&HashMap<String, Value>> {
        self.metadata.as_ref()
    }
}
