use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct CardChannelProperties {
    skip_three_d_secure: Option<bool>,
    success_return_url: Option<String>,
    failure_return_url: Option<String>,
    cardonfile_type: Option<String>,
    expires_at: Option<DateTime<Utc>>
}
impl CardChannelProperties {
    pub fn new() -> CardChannelProperties {
        CardChannelProperties {
            skip_three_d_secure: None,
            success_return_url: None,
            failure_return_url: None,
            cardonfile_type: None,
            expires_at: None
        }
    }
    pub fn is_skip_three_d_secure(&self) -> bool {
        self.skip_three_d_secure.unwrap_or(false)
    }
    pub fn set_skip_three_d_secure(&mut self, skip: bool) -> &mut Self {
        self.skip_three_d_secure = Some(skip);
        self
    }
    pub fn get_success_return_url(&self) -> Option<&String> {
        self.success_return_url.as_ref()
    }
    pub fn set_success_return_url(&mut self, url: String) -> &mut Self {
        self.success_return_url = Some(url);
        self
    }
    pub fn get_failure_return_url(&self) -> Option<&String> {
        self.failure_return_url.as_ref()
    }
    pub fn set_failure_return_url(&mut self, url: String) -> &mut Self {
        self.failure_return_url = Some(url);
        self
    }
    pub fn get_cardonfile_type(&self) -> Option<&String> {
        self.cardonfile_type.as_ref()
    }
    pub fn set_cardonfile_type(&mut self, cardonfile_type: String) -> &mut Self {
        self.cardonfile_type = Some(cardonfile_type);
        self
    }
    pub fn get_expires_at(&self) -> Option<&DateTime<Utc>> {
        self.expires_at.as_ref()
    }
    pub fn set_expires_at(&mut self, expires_at: DateTime<Utc>) -> &mut Self {
        self.expires_at = Some(expires_at);
        self
    }
    pub fn build(&self) -> Self {
        self.clone()
    }
}