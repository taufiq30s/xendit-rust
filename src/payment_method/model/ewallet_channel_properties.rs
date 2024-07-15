use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone)]
pub struct EWalletChannelProperties {
    success_return_url: Option<String>,
    failure_return_url: Option<String>,
    cancel_return_url: Option<String>,
    pending_return_url: Option<String>,
    mobile_number: Option<String>,
    redeem_points: Option<String>,
    cashtag: Option<String>
}
impl EWalletChannelProperties {
    pub fn new() -> Self {
        Self {
            success_return_url: None,
            failure_return_url: None,
            cancel_return_url: None,
            pending_return_url: None,
            mobile_number: None,
            redeem_points: None,
            cashtag: None
        }
    }
    pub fn get_success_return_url(&self) -> Option<&String> {
        self.success_return_url.as_ref()
    }
    pub fn set_success_return_url(&mut self, success_return_url: String) -> &mut Self {
        self.success_return_url = Some(success_return_url);
        self
    }
    pub fn get_failure_return_url(&self) -> Option<&String> {
        self.failure_return_url.as_ref()
    }
    pub fn set_failure_return_url(&mut self, failure_return_url: String) -> &mut Self {
        self.failure_return_url = Some(failure_return_url);
        self
    }
    pub fn get_cancel_return_url(&self) -> Option<&String> {
        self.cancel_return_url.as_ref()
    }
    pub fn set_cancel_return_url(&mut self, cancel_return_url: String) -> &mut Self {
        self.cancel_return_url = Some(cancel_return_url);
        self
    }
    pub fn get_pending_return_url(&self) -> Option<&String> {
        self.pending_return_url.as_ref()
    }
    pub fn set_pending_return_url(&mut self, pending_return_url: String) -> &mut Self {
        self.pending_return_url = Some(pending_return_url);
        self
    }
    pub fn get_mobile_number(&self) -> Option<&String> {
        self.mobile_number.as_ref()
    }
    pub fn set_mobile_number(&mut self, mobile_number: String) -> &mut Self {
        self.mobile_number = Some(mobile_number);
        self
    }
    pub fn get_redeem_points(&self) -> Option<&String> {
        self.redeem_points.as_ref()
    }
    pub fn set_redeem_points(&mut self, redeem_points: String) -> &mut Self {
        self.redeem_points = Some(redeem_points);
        self
    }
    pub fn get_cashtag(&self) -> Option<&String> {
        self.cashtag.as_ref()
    }
    pub fn set_cashtag(&mut self, cashtag: String) -> &mut Self {
        self.cashtag = Some(cashtag);
        self
    }
    pub fn build(&mut self) -> Self {
        self.clone()
    }
}