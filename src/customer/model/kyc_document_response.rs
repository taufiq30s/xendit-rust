use serde::Deserialize;

use super::{KYCDocumentSubType, KYCDocumentType};

#[derive(Deserialize)]
pub struct KYCDocumentResponse {
    country: String,
    type_field: KYCDocumentType,
    sub_type: KYCDocumentSubType,
    document_name: Option<String>,
    document_number: Option<String>,
    expires_at: Option<String>,
    holder_name: Option<String>,
    document_images: Vec<String>,
}

impl KYCDocumentResponse {
    pub fn get_country(&self) -> &str {
        &self.country
    }

    pub fn get_type(&self) -> &KYCDocumentType {
        &self.type_field
    }

    pub fn get_sub_type(&self) -> &KYCDocumentSubType {
        &self.sub_type
    }

    pub fn get_document_name(&self) -> Option<&String> {
        self.document_name.as_ref()
    }

    pub fn get_document_number(&self) -> Option<&String> {
        self.document_number.as_ref()
    }

    pub fn get_expires_at(&self) -> Option<&String> {
        self.expires_at.as_ref()
    }

    pub fn get_holder_name(&self) -> Option<&String> {
        self.holder_name.as_ref()
    }

    pub fn get_document_images(&self) -> &Vec<String> {
        &self.document_images
    }
}
