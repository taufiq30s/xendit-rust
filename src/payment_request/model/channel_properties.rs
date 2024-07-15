use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct ChannelProperties {
    success_return_url: Option<String>,
    failure_return_url: Option<String>,
    cancel_return_url: Option<String>,
    redeem_points: Option<String>,
    require_auth: Option<bool>,
    merchant_id_tag: Option<String>,
    cardonfile_type: Option<String>,
    cvv: Option<String>,
}

impl ChannelProperties {
    pub fn new() -> Self {
        ChannelProperties {
            success_return_url: None,
            failure_return_url: None,
            cancel_return_url: None,
            redeem_points: None,
            require_auth: None,
            merchant_id_tag: None,
            cardonfile_type: None,
            cvv: None,
        }
    }
    pub fn set_success_return_url(&mut self, url: String) -> &mut Self {
        self.success_return_url = Some(url);
        self
    }
    pub fn get_success_return_url(&self) -> Option<&String> {
        self.success_return_url.as_ref()
    }
    pub fn set_failure_return_url(&mut self, url: String) -> &mut Self {
        self.failure_return_url = Some(url);
        self
    }
    pub fn get_failure_return_url(&self) -> Option<&String> {
        self.failure_return_url.as_ref()
    }
    pub fn set_cancel_return_url(&mut self, url: String) -> &mut Self {
        self.cancel_return_url = Some(url);
        self
    }
    pub fn get_cancel_return_url(&self) -> Option<&String> {
        self.cancel_return_url.as_ref()
    }
    pub fn set_redeem_points(&mut self, points: String) -> &mut Self {
        self.redeem_points = Some(points);
        self
    }
    pub fn get_redeem_points(&self) -> Option<&String> {
        self.redeem_points.as_ref()
    }
    pub fn set_require_auth(&mut self, require_auth: bool) -> &mut Self {
        self.require_auth = Some(require_auth);
        self
    }
    pub fn get_require_auth(&self) -> Option<bool> {
        self.require_auth
    }
    pub fn set_merchant_id_tag(&mut self, tag: String) -> &mut Self {
        self.merchant_id_tag = Some(tag);
        self
    }
    pub fn get_merchant_id_tag(&self) -> Option<&String> {
        self.merchant_id_tag.as_ref()
    }
    pub fn set_cardonfile_type(&mut self, cardonfile_type: String) -> &mut Self {
        self.cardonfile_type = Some(cardonfile_type);
        self
    }
    pub fn get_cardonfile_type(&self) -> Option<&String> {
        self.cardonfile_type.as_ref()
    }
    pub fn set_cvv(&mut self, cvv: String) -> &mut Self {
        self.cvv = Some(cvv);
        self
    }
    pub fn get_cvv(&self) -> Option<&String> {
        self.cvv.as_ref()
    }
    pub fn build(&self) -> ChannelProperties {
        self.clone()
    }
}
