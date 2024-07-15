use serde::Deserialize;

#[derive(Deserialize)]
pub struct Action {
    action: String,
    method: String,
    url_type: String,
    url: String,
}
impl Action {
    pub fn get_action(&self) -> &str {
        &self.action
    }
    pub fn get_method(&self) -> &str {
        &self.method
    }
    pub fn get_url_type(&self) -> &str {
        &self.url_type
    }
    pub fn get_url(&self) -> &str {
        &self.url
    }
}
