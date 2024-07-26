use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Clone)]
pub struct ReceiptNotification {
    email_to: Option<Vec<String>>,
    email_cc: Option<Vec<String>>,
    email_bcc: Option<Vec<String>>,
}
impl ReceiptNotification {
    pub fn new() -> Self {
        Self {
            email_to: None,
            email_cc: None,
            email_bcc: None,
        }
    }
    pub fn get_email_to(&self) -> Option<&Vec<String>> {
        self.email_to.as_ref()
    }
    pub fn set_email_to(&mut self, email_to: Vec<String>) {
        self.email_to = Some(email_to);
    }
    pub fn get_email_cc(&self) -> Option<&Vec<String>> {
        self.email_cc.as_ref()
    }
    pub fn set_email_cc(&mut self, email_cc: Vec<String>) {
        self.email_cc = Some(email_cc);
    }
    pub fn get_email_bcc(&self) -> Option<&Vec<String>> {
        self.email_bcc.as_ref()
    }
    pub fn set_email_bcc(&mut self, email_bcc: Vec<String>) {
        self.email_bcc = Some(email_bcc);
    }
    pub fn build(&self) -> ReceiptNotification {
        self.clone()
    }
}
