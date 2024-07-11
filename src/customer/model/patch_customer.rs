use serde::Serialize;
use serde_with::skip_serializing_none;
use std::collections::HashMap;

use super::{AddressRequest, BusinessDetail, EndCustomerStatus, IdentityAccountRequest, IndividualDetail, KYCDocumentRequest};

#[skip_serializing_none]
#[derive(Serialize, Clone)]
pub struct PatchCustomer {
    client_name: Option<String>,
    reference_id: Option<String>,
    individual_detail: Option<IndividualDetail>,
    business_detail: Option<BusinessDetail>,
    description: Option<String>,
    email: Option<String>,
    mobile_number: Option<String>,
    phone_number: Option<String>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    addresses: Option<Vec<AddressRequest>>,
    identity_accounts: Option<Vec<IdentityAccountRequest>>,
    kyc_documents: Option<Vec<KYCDocumentRequest>>,
    status: Option<EndCustomerStatus>,
}
impl PatchCustomer {
    pub fn new() -> Self {
        PatchCustomer {
            client_name: None,
            reference_id: None,
            individual_detail: None,
            business_detail: None,
            description: None,
            email: None,
            mobile_number: None,
            phone_number: None,
            metadata: None,
            addresses: None,
            identity_accounts: None,
            kyc_documents: None,
            status: None,
        }
    }

    pub fn set_client_name(&mut self, client_name: String) -> &mut Self {
        self.client_name = Some(client_name);
        self
    }
    pub fn get_client_name(&self) -> Option<&String> {
        self.client_name.as_ref()
    }
    pub fn set_reference_id(&mut self, reference_id: String) -> &mut Self {
        self.reference_id = Some(reference_id);
        self
    }
    pub fn get_reference_id(&self) -> Option<&String> {
        self.reference_id.as_ref()
    }
    pub fn set_individual_detail(&mut self, individual_detail: IndividualDetail) -> &mut Self {
        self.individual_detail = Some(individual_detail);
        self
    }
    pub fn get_individual_detail(&self) -> Option<&IndividualDetail> {
        self.individual_detail.as_ref()
    }
    pub fn set_business_detail(&mut self, business_detail: BusinessDetail) -> &mut Self {
        self.business_detail = Some(business_detail);
        self
    }
    pub fn get_business_detail(&self) -> Option<&BusinessDetail> {
        self.business_detail.as_ref()
    }
    pub fn set_description(&mut self, description: String) -> &mut Self {
        self.description = Some(description);
        self
    }
    pub fn get_description(&self) -> Option<&String> {
        self.description.as_ref()
    }
    pub fn set_email(&mut self, email: String) -> &mut Self {
        self.email = Some(email);
        self
    }
    pub fn get_email(&self) -> Option<&String> {
        self.email.as_ref()
    }
    pub fn set_mobile_number(&mut self, mobile_number: String) -> &mut Self {
        self.mobile_number = Some(mobile_number);
        self
    }
    pub fn get_mobile_number(&self) -> Option<&String> {
        self.mobile_number.as_ref()
    }
    pub fn set_phone_number(&mut self, phone_number: String) -> &mut Self {
        self.phone_number = Some(phone_number);
        self
    }
    pub fn get_phone_number(&self) -> Option<&String> {
        self.phone_number.as_ref()
    }
    pub fn set_metadata(&mut self, metadata: HashMap<String, serde_json::Value>) -> &mut Self {
        self.metadata = Some(metadata);
        self
    }
    pub fn get_metadata(&self) -> Option<&HashMap<String, serde_json::Value>> {
        self.metadata.as_ref()
    }
    pub fn set_addresses(&mut self, addresses: Vec<AddressRequest>) -> &mut Self {
        self.addresses = Some(addresses);
        self
    }
    pub fn set_identity_accounts(&mut self, identity_accounts: Vec<IdentityAccountRequest>) -> &mut Self {
        self.identity_accounts = Some(identity_accounts);
        self
    }
    pub fn get_identity_accounts(&self) -> Option<&Vec<IdentityAccountRequest>> {
        self.identity_accounts.as_ref()
    }
    pub fn set_kyc_documents(&mut self, kyc_documents: Vec<KYCDocumentRequest>) -> &mut Self {
        self.kyc_documents = Some(kyc_documents);
        self
    }
    pub fn get_kyc_documents(&self) -> Option<&Vec<KYCDocumentRequest>> {
        self.kyc_documents.as_ref()
    }
    pub fn set_status(&mut self, status: EndCustomerStatus) -> &mut Self {
        self.status = Some(status);
        self
    }
    pub fn get_status(&self) -> Option<&EndCustomerStatus> {
        self.status.as_ref()
    }    
    pub fn build(&self) -> Self {
        self.clone()
    }
}