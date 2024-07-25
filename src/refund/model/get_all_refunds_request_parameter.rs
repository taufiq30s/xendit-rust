use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct GetAllRefundsRequestParams {
    payment_request_id: Option<String>,
    invoice_id: Option<String>,
    payment_method_type: Option<String>,
    channel_code: Option<String>,
    limit: Option<u64>,
    after_id: Option<String>,
    before_id: Option<String>,
}
impl GetAllRefundsRequestParams {
    pub fn new() -> Self {
        Self {
            payment_request_id: None,
            invoice_id: None,
            payment_method_type: None,
            channel_code: None,
            limit: None,
            after_id: None,
            before_id: None,
        }
    }
    pub fn get_payment_request_id(&self) -> Option<&String> {
        self.payment_request_id.as_ref()
    }
    pub fn set_payment_request_id(&mut self, payment_request_id: String) -> &mut Self {
        self.payment_request_id = Some(payment_request_id);
        self
    }
    pub fn get_invoice_id(&self) -> Option<&String> {
        self.invoice_id.as_ref()
    }
    pub fn set_invoice_id(&mut self, invoice_id: String) -> &mut Self {
        self.invoice_id = Some(invoice_id);
        self
    }
    pub fn get_payment_method_type(&self) -> Option<&String> {
        self.payment_method_type.as_ref()
    }
    pub fn set_payment_method_type(&mut self, payment_method_type: String) -> &mut Self {
        self.payment_method_type = Some(payment_method_type);
        self
    }
    pub fn get_channel_code(&self) -> Option<&String> {
        self.channel_code.as_ref()
    }
    pub fn set_channel_code(&mut self, channel_code: String) -> &mut Self {
        self.channel_code = Some(channel_code);
        self
    }
    pub fn get_limit(&self) -> Option<&u64> {
        self.limit.as_ref()
    }
    pub fn set_limit(&mut self, limit: u64) -> &mut Self {
        self.limit = Some(limit);
        self
    }
    pub fn get_after_id(&self) -> Option<&String> {
        self.after_id.as_ref()
    }
    pub fn set_after_id(&mut self, after_id: String) -> &mut Self {
        self.after_id = Some(after_id);
        self
    }
    pub fn get_before_id(&self) -> Option<&String> {
        self.before_id.as_ref()
    }
    pub fn set_before_id(&mut self, before_id: String) -> &mut Self {
        self.before_id = Some(before_id);
        self
    }
    pub fn build(&self) -> GetAllRefundsRequestParams {
        self.clone()
    }
}