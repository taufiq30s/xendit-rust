use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct InvoiceFee {
    r#type: String,
    value: f64,
}
impl InvoiceFee {
    pub fn new(r#type: String, value: f64) -> Self {
        Self { r#type, value }
    }
    pub fn get_type(&self) -> &str {
        &self.r#type
    }
    pub fn set_type(&mut self, r#type: String) -> &mut Self {
        self.r#type = r#type;
        self
    }
    pub fn get_value(&self) -> f64 {
        self.value
    }
    pub fn set_value(&mut self, value: f64) -> &mut Self {
        self.value = value;
        self
    }
    pub fn build(&self) -> Self {
        self.clone()
    }
}