use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct PaymentMethodExpireParams {
    pub success_return_url: Option<String>,
    pub failure_return_url: Option<String>,
}
impl PaymentMethodExpireParams {
    pub fn new(success_return_url: String, failure_return_url: String) -> Self {
        Self {
            success_return_url: Some(success_return_url),
            failure_return_url: Some(failure_return_url),
        }
    }
    pub fn get_success_return_url(&self) -> Option<String> {
        self.success_return_url.clone()
    }
    pub fn set_success_return_url(&mut self, success_return_url: String) -> &mut Self {
        self.success_return_url = Some(success_return_url);
        self
    }
    pub fn get_failure_return_url(&self) -> Option<String> {
        self.failure_return_url.clone()
    }
    pub fn set_failure_return_url(&mut self, failure_return_url: String) -> &mut Self {
        self.failure_return_url = Some(failure_return_url);
        self
    }
    pub fn build(self) -> Self {
        self.clone()
    }
}