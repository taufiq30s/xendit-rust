use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct InstallmentConfiguration {
    allow_full_payment: bool,
    allowed_terms: Vec<AllowedTerm>
}
impl InstallmentConfiguration {
    pub fn new(allow_full_payment: bool, allowed_terms: Vec<AllowedTerm>) -> Self {
        InstallmentConfiguration {
            allow_full_payment,
            allowed_terms
        }
    }
    pub fn get_allow_full_payment(&self) -> bool {
        self.allow_full_payment
    }
    pub fn set_allow_full_payment(&mut self, allow_full_payment: bool) -> &mut Self {
        self.allow_full_payment = allow_full_payment;
        self
    }
    pub fn get_allowed_terms(&self) -> &Vec<AllowedTerm> {
        &self.allowed_terms
    }
    pub fn set_allowed_terms(&mut self, allowed_terms: Vec<AllowedTerm>) -> &mut Self {
        self.allowed_terms = allowed_terms;
        self
    }
    pub fn build(&self) -> Self {
        self.clone()
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AllowedTerm {
    issuer: String,
    terms: Vec<u64>
}
impl AllowedTerm {
    pub fn new(issuer: String, terms: Vec<u64>) -> Self {
        AllowedTerm {
            issuer,
            terms
        }
    }
    pub fn get_issuer(&self) -> &str {
        &self.issuer
    }
    pub fn set_issuer(&mut self, issuer: String) -> &mut Self {
        self.issuer = issuer;
        self
    }
    pub fn get_terms(&self) -> &Vec<u64> {
        &self.terms
    }
    pub fn set_terms(&mut self, terms: Vec<u64>) -> &mut Self {
        self.terms = terms;
        self
    }
    pub fn build(&self) -> Self {
        self.clone()
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ChannelProperties {
    allowed_bins: Option<Vec<String>>,
    installment_configuration: Option<InstallmentConfiguration>
}
impl ChannelProperties {
    pub fn new() -> Self {
        Self {
            allowed_bins: None,
            installment_configuration: None
        }
    }
    pub fn get_allowed_bins(&self) -> &Option<Vec<String>> {
        &self.allowed_bins
    }
    pub fn set_allowed_bins(&mut self, bins: Vec<String>) -> &mut Self {
        self.allowed_bins = Some(bins);
        self
    }
    pub fn get_installment_configuration(&self) -> &Option<InstallmentConfiguration> {
        &self.installment_configuration
    }
    pub fn set_installment_configuration(&mut self, config: InstallmentConfiguration) -> &mut Self {
        self.installment_configuration = Some(config);
        self
    }
    pub fn build(&self) -> Self {
        self.clone()
    }
}