use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::{AddressRequest, BusinessDetail, IdentityAccountRequest, IndividualDetail, KYCDocumentRequest};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CustomerType {
    Individual,
    Business
}

#[skip_serializing_none]
#[derive(Serialize, Clone)]
pub struct CustomerRequestBody {
    client_name: Option<String>,
    reference_id: String,
    r#type: CustomerType,
    individual_detail: Option<IndividualDetail>,
    business_detail: Option<BusinessDetail>,
    description: Option<String>,
    email: Option<String>,
    mobile_number: Option<String>,
    phone_number: Option<String>,
    addresses: Option<Vec<AddressRequest>>,
    identity_accounts: Option<Vec<IdentityAccountRequest>>,
    kyc_documents: Option<Vec<KYCDocumentRequest>>,
    metadata: Option<HashMap<String, String>>
}
impl CustomerRequestBody {
    pub fn new(reference_id: String, r#type: CustomerType) -> Self {
        CustomerRequestBody {
            client_name: None,
            reference_id,
            r#type,
            individual_detail: None,
            business_detail: None,
            description: None,
            email: None,
            mobile_number: None,
            phone_number: None,
            addresses: None,
            identity_accounts: None,
            kyc_documents: None,
            metadata: None
        }
    }
    pub fn set_client_name(&mut self, client_name: String) -> &mut Self {
        self.client_name = Some(client_name);
        self
    }
    pub fn set_individual_detail(&mut self, individual_detail: IndividualDetail) -> &mut Self {
        self.individual_detail = Some(individual_detail);
        self
    }
    pub fn set_business_detail(&mut self, business_detail: BusinessDetail) -> &mut Self {
        self.business_detail = Some(business_detail);
        self
    }
    pub fn set_description(&mut self, description: String) -> &mut Self {
        self.description = Some(description);
        self
    }
    pub fn set_email(&mut self, email: String) -> &mut Self {
        self.email = Some(email);
        self
    }
    pub fn set_mobile_number(&mut self, mobile_number: String) -> &mut Self {
        self.mobile_number = Some(mobile_number);
        self
    }
    pub fn set_phone_number(&mut self, phone_number: String) -> &mut Self {
        self.phone_number = Some(phone_number);
        self
    }
    pub fn set_addresses(&mut self, addresses: Vec<AddressRequest>) -> &mut Self {
        self.addresses = Some(addresses);
        self
    }
    pub fn set_identity_accounts(&mut self, identity_accounts: Vec<IdentityAccountRequest>) -> &mut Self {
        self.identity_accounts = Some(identity_accounts);
        self
    }
    pub fn set_kyc_documents(&mut self, kyc_documents: Vec<KYCDocumentRequest>) -> &mut Self {
        self.kyc_documents = Some(kyc_documents);
        self
    }
    pub fn set_metadata(&mut self, metadata: HashMap<String, String>) -> &mut Self {
        self.metadata = Some(metadata);
        self
    }
    pub fn build(&mut self) -> CustomerRequestBody {
        self.clone()
    }
}