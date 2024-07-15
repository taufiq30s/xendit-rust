use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct BillingInformation {
    country: String,
    street_line_1: Option<String>,
    street_line_2: Option<String>,
    city: Option<String>,
    province_state: Option<String>,
    postal_code: Option<String>,
}
impl BillingInformation {
    pub fn new(country: String) -> Self {
        Self {
            country,
            street_line_1: None,
            street_line_2: None,
            city: None,
            province_state: None,
            postal_code: None,
        }
    }
    pub fn get_country(&self) -> &str {
        &self.country
    }
    pub fn set_country(&mut self, country: String) {
        self.country = country;
    }
    pub fn get_street_line_1(&self) -> Option<&String> {
        self.street_line_1.as_ref()
    }
    pub fn set_street_line_1(&mut self, street_line_1: String) {
        self.street_line_1 = Some(street_line_1);
    }
    pub fn get_street_line_2(&self) -> Option<&String> {
        self.street_line_2.as_ref()
    }
    pub fn set_street_line_2(&mut self, street_line_2: String) {
        self.street_line_2 = Some(street_line_2);
    }
    pub fn get_city(&self) -> Option<&String> {
        self.city.as_ref()
    }
    pub fn set_city(&mut self, city: String) {
        self.city = Some(city);
    }
    pub fn get_province_state(&self) -> Option<&String> {
        self.province_state.as_ref()
    }
    pub fn set_province_state(&mut self, province_state: String) {
        self.province_state = Some(province_state);
    }
    pub fn get_postal_code(&self) -> Option<&String> {
        self.postal_code.as_ref()
    }
    pub fn set_postal_code(&mut self, postal_code: String) {
        self.postal_code = Some(postal_code);
    }
    pub fn build(&self) -> Self {
        self.clone()
    }
}