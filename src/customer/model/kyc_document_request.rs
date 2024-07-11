use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

// KYC Document Request
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum KYCDocumentType {
    BirthCertificate,
    BankStatement,
    DrivingLicense,
    IdentityCard,
    Passport,
    Visa,
    BusinessRegistration,
    BusinessLicense,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum KYCDocumentSubType {
    NationalId,
    ConsularId,
    VoterId,
    PostalId,
    ResidencePermit,
    TaxId,
    StudentId,
    MilitaryId,
    MedicalId,
    Others,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone)]
pub struct KYCDocumentRequest {
    pub country: String,
    pub r#type: KYCDocumentType,
    pub sub_type: Option<KYCDocumentSubType>,
    pub document_name: Option<String>,
    pub document_number: Option<String>,
    pub expires_at: Option<NaiveDate>,
    pub holder_name: Option<String>,
    pub document_images: Option<Vec<String>>
}
impl KYCDocumentRequest {
    pub fn new(country: String, r#type: KYCDocumentType) -> Self {
        KYCDocumentRequest {
            country,
            r#type,
            sub_type: None,
            document_name: None,
            document_number: None,
            expires_at: None,
            holder_name: None,
            document_images: None
        }
    }
    pub fn set_country(&mut self, country: String) -> &mut Self {
        self.country = country;
        self
    }
    pub fn get_country(&self) -> &String {
        &self.country
    }
    pub fn set_type(&mut self, r#type: KYCDocumentType) -> &mut Self {
        self.r#type = r#type;
        self
    }
    pub fn get_type(&self) -> &KYCDocumentType {
        &self.r#type
    }
    pub fn set_sub_type(&mut self, sub_type: KYCDocumentSubType) -> &mut Self {
        self.sub_type = Some(sub_type);
        self
    }
    pub fn get_sub_type(&self) -> Option<&KYCDocumentSubType> {
        self.sub_type.as_ref()
    }
    pub fn set_document_name(&mut self, document_name: String) -> &mut Self {
        self.document_name = Some(document_name);
        self
    }
    pub fn get_document_name(&self) -> Option<&String> {
        self.document_name.as_ref()
    }
    pub fn set_document_number(&mut self, document_number: String) -> &mut Self {
        self.document_number = Some(document_number);
        self
    }
    pub fn get_document_number(&self) -> Option<&String> {
        self.document_number.as_ref()
    }
    pub fn set_expires_at(&mut self, expires_at: &str) -> &mut Self {
        self.expires_at = Some(NaiveDate::parse_from_str(expires_at, "%Y-%m-%d").unwrap());
        self
    }
    pub fn get_expires_at(&self) -> Option<&NaiveDate> {
        self.expires_at.as_ref()
    }
    pub fn set_holder_name(&mut self, holder_name: String) -> &mut Self {
        self.holder_name = Some(holder_name);
        self
    }
    pub fn get_holder_name(&self) -> Option<&String> {
        self.holder_name.as_ref()
    }
    pub fn set_document_images(&mut self, document_images: Vec<String>) -> &mut Self {
        self.document_images = Some(document_images);
        self
    }
    pub fn get_document_images(&self) -> Option<&Vec<String>> {
        self.document_images.as_ref()
    }
    pub fn build(&mut self) -> KYCDocumentRequest {
        self.clone()
    }
}