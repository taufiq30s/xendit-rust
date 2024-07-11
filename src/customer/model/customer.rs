use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{Address, BusinessDetail, CustomerType, IdentityAccountResponse, IndividualDetail, KYCDocumentResponse};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EndCustomerStatus {
    Active,
    Inactive,
    Pending,
    Blocked,
    Deleted
}

// Responses Body
#[derive(Deserialize)]
pub struct Customer {
    id: String,
    reference_id: String,
    r#type: CustomerType,
    individual_detail: Option<IndividualDetail>,
    business_detail: Option<BusinessDetail>,
    description: Option<String>,
    email: Option<String>,
    mobile_number: Option<String>,
    phone_number: Option<String>,
    hashed_phone_number: Option<String>,
    addresses: Vec<Address>,
    identity_accounts: Vec<IdentityAccountResponse>,
    kyc_documents: Vec<KYCDocumentResponse>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    status: Option<EndCustomerStatus>,
    domicile_of_registration: Option<String>,
    date_of_registration: Option<String>,
    created: DateTime<Utc>,
    updated: DateTime<Utc>,
}
impl Customer {
    pub fn get_id(&self) -> &str {
        &self.id
    }
    pub fn get_reference_id(&self) -> &str {
        &self.reference_id
    }
    pub fn get_type(&self) -> &CustomerType {
        &self.r#type
    }
    pub fn get_individual_detail(&self) -> Option<&IndividualDetail> {
        self.individual_detail.as_ref()
    }
    pub fn get_business_detail(&self) -> Option<&BusinessDetail> {
        self.business_detail.as_ref()
    }
    pub fn get_description(&self) -> Option<&str> {
        self.description.as_deref()
    }
    pub fn get_email(&self) -> Option<&str> {
        self.email.as_deref()
    }
    pub fn get_mobile_number(&self) -> Option<&str> {
        self.mobile_number.as_deref()
    }
    pub fn get_phone_number(&self) -> Option<&str> {
        self.phone_number.as_deref()
    }
    pub fn get_hashed_phone_number(&self) -> Option<&str> {
        self.hashed_phone_number.as_deref()
    }
    pub fn get_addresses(&self) -> &[Address] {
        &self.addresses
    }
    pub fn get_identity_accounts(&self) -> &[IdentityAccountResponse] {
        &self.identity_accounts
    }
    pub fn get_kyc_documents(&self) -> &[KYCDocumentResponse] {
        &self.kyc_documents
    }
    pub fn get_metadata(&self) -> Option<&HashMap<String, serde_json::Value>> {
        self.metadata.as_ref()
    }
    pub fn get_status(&self) -> Option<&EndCustomerStatus> {
        self.status.as_ref()
    }
    pub fn get_domicile_of_registration(&self) -> Option<&str> {
        self.domicile_of_registration.as_deref()
    }
    pub fn get_date_of_registration(&self) -> Option<&str> {
        self.date_of_registration.as_deref()
    }
    pub fn get_created(&self) -> &DateTime<Utc> {
        &self.created
    }
    pub fn get_updated(&self) -> &DateTime<Utc> {
        &self.updated
    }    
}