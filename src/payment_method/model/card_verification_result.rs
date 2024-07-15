use serde::Deserialize;

use crate::payment_method::CardVerificationResultsThreeDSecure;

#[derive(Deserialize)]
pub struct CardVerificationResults {
    three_d_secure: Option<CardVerificationResultsThreeDSecure>,
    cvv_result: Option<String>,
    address_verification_result: Option<String>,
}
impl CardVerificationResults {
    pub fn get_three_d_secure(&self) -> Option<&CardVerificationResultsThreeDSecure> {
        self.three_d_secure.as_ref()
    }
    pub fn get_cvv_result(&self) -> Option<&String> {
        self.cvv_result.as_ref()
    }
    pub fn get_address_verification_result(&self) -> Option<&String> {
        self.address_verification_result.as_ref()
    }
}