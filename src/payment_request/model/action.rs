use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UriType {
    Api,
    Web,
    Mobile,
    Deeplink,
    #[serde(other)]
    Invalid
}

#[derive(Deserialize)]
pub struct Action {
    action: String,
    uri_type: Option<UriType>,
    method: Option<String>,
    uri: Option<String>,
    qr_code: Option<String>,
}
impl Action {
    pub fn get_action(&self) -> &str {
        &self.action
    }
    pub fn get_uri_type(&self) -> Option<&UriType> {
        self.uri_type.as_ref()
    }
    pub fn get_method(&self) -> Option<&str> {
        self.method.as_deref()
    }
    pub fn get_uri(&self) -> Option<&str> {
        self.uri.as_deref()
    }
    pub fn get_qr_code(&self) -> Option<&str> {
        self.qr_code.as_deref()
    }
}