use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct GetPayoutsRequestParams {
    reference_id: String,
    limit: Option<f32>,
    after_id: Option<String>,
    before_id: Option<String>,
}
impl GetPayoutsRequestParams {
    pub fn new(reference_id: String) -> Self {
        Self {
            reference_id,
            limit: None,
            after_id: None,
            before_id: None,
        }
    }
    pub fn get_reference_id(&self) -> &str {
        &self.reference_id
    }
    pub fn set_reference_id(&mut self, reference_id: String) -> &mut Self {
        self.reference_id = reference_id;
        self
    }
    pub fn get_limit(&self) -> Option<&f32> {
        self.limit.as_ref()
    }
    pub fn set_limit(&mut self, limit: f32) -> &mut Self {
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
    pub fn build(&self) -> Self {
        self.clone()
    }
}