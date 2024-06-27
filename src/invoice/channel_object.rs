use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct InstallmentConfiguration {
    pub allow_full_payment: bool,
    pub allowed_terms: Vec<AllowedTerm>
}
impl InstallmentConfiguration {
    pub fn new(allow_full_payment: bool, allowed_terms: Vec<AllowedTerm>) -> Self {
        InstallmentConfiguration {
            allow_full_payment,
            allowed_terms
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AllowedTerm {
    pub issuer: String,
    pub terms: Vec<u64>
}
impl AllowedTerm {
    pub fn new(issuer: String, terms: Vec<u64>) -> Self {
        AllowedTerm {
            issuer,
            terms
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Channel {
    pub allowed_bins: Option<Vec<String>>,
    pub installment_configuration: Option<InstallmentConfiguration>
}
impl Channel {
    pub fn new() -> Self {
        Channel {
            allowed_bins: None,
            installment_configuration: None
        }
    }
    pub fn set_allowed_bins(&mut self, bins: Vec<String>) -> &mut Self {
        self.allowed_bins = Some(bins);
        self
    }
    pub fn set_installment_configuration(&mut self, config: InstallmentConfiguration) -> &mut Self {
        self.installment_configuration = Some(config);
        self
    }
    pub fn build(&self) -> Channel {
        Channel {
            allowed_bins: self.allowed_bins.clone(),
            installment_configuration: self.installment_configuration.clone()
        }
    }
}