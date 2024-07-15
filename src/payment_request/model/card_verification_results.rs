use serde::Deserialize;

#[derive(Deserialize)]
pub struct CardVerificationResultThreeDSecure {
    three_d_secure_flow: Option<String>,
    eci_code: Option<String>,
    three_d_secure_result: Option<String>,
    three_d_secure_result_reason: Option<String>,
    three_d_secure_version: Option<String>,
}

impl CardVerificationResultThreeDSecure {
    pub fn get_three_d_secure_flow(&self) -> Option<&String> {
        self.three_d_secure_flow.as_ref()
    }
    pub fn get_eci_code(&self) -> Option<&String> {
        self.eci_code.as_ref()
    }
    pub fn get_three_d_secure_result(&self) -> Option<&String> {
        self.three_d_secure_result.as_ref()
    }
    pub fn get_three_d_secure_result_reason(&self) -> Option<&String> {
        self.three_d_secure_result_reason.as_ref()
    }
    pub fn get_three_d_secure_version(&self) -> Option<&String> {
        self.three_d_secure_version.as_ref()
    }
}

#[derive(Deserialize)]
pub struct CardVerificationResult {
    three_d_secure: Option<CardVerificationResultThreeDSecure>,
    cvv_result: Option<String>,
    address_verification_result: Option<String>,
}
impl CardVerificationResult {
    pub fn get_three_d_secure(&self) -> Option<&CardVerificationResultThreeDSecure> {
        self.three_d_secure.as_ref()
    }
    pub fn get_cvv_result(&self) -> Option<&String> {
        self.cvv_result.as_ref()
    }
    pub fn get_address_verification_result(&self) -> Option<&String> {
        self.address_verification_result.as_ref()
    }
}