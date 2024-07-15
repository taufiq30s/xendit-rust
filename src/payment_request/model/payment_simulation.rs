use serde::Deserialize;

#[derive(Deserialize)]
pub struct PaymentSimulation {
    status: String,
    message: String
}
impl PaymentSimulation {
    pub fn get_status(&self) -> &str {
        &self.status
    }
    pub fn get_message(&self) -> &str {
        &self.message
    }
}