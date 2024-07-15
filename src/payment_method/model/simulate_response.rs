use serde::Deserialize;

#[derive(Deserialize)]
pub struct SimulateResponse {
    status: String,
    message: String,
}
impl SimulateResponse {
    pub fn get_status(&self) -> &str {
        &self.status
    }
    pub fn get_message(&self) -> &str {
        &self.message
    }
}