use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::AddressCategory;

// Customer Address Request
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AddressStatus {
    Active,
    Deleted
}

#[skip_serializing_none]
#[derive(Serialize, Clone)]
pub struct AddressRequest {
    country: String,
    category: Option<AddressCategory>,
    province_state: Option<String>,
    city: Option<String>,
    suburb: Option<String>,
    postal_code: Option<String>,
    street_line1: Option<String>,
    line_2: Option<String>,
    status: Option<AddressStatus>,
    is_primary: Option<bool>
}
impl AddressRequest {
    pub fn new(country: String) -> Self {
        AddressRequest {
            country,
            category: None,
            province_state: None,
            city: None,
            suburb: None,
            postal_code: None,
            street_line1: None,
            line_2: None,
            status: None,
            is_primary: None
        }
    }
    pub fn set_country(&mut self, country: String) -> &mut Self {
        self.country = country;
        self
    }
    pub fn get_country(&self) -> &String {
        &self.country
    }
    pub fn set_category(&mut self, category: AddressCategory) -> &mut Self {
        self.category = Some(category);
        self
    }
    pub fn get_category(&self) -> Option<&AddressCategory> {
        self.category.as_ref()
    }
    pub fn set_province_state(&mut self, province_state: String) -> &mut Self {
        self.province_state = Some(province_state);
        self
    }
    pub fn get_province_state(&self) -> Option<&String> {
        self.province_state.as_ref()
    }
    pub fn set_city(&mut self, city: String) -> &mut Self {
        self.city = Some(city);
        self
    }
    pub fn get_city(&self) -> Option<&String> {
        self.city.as_ref()
    }
    pub fn set_suburb(&mut self, suburb: String) -> &mut Self {
        self.suburb = Some(suburb);
        self
    }
    pub fn get_suburb(&self) -> Option<&String> {
        self.suburb.as_ref()
    }
    pub fn set_postal_code(&mut self, postal_code: String) -> &mut Self {
        self.postal_code = Some(postal_code);
        self
    }
    pub fn get_postal_code(&self) -> Option<&String> {
        self.postal_code.as_ref()
    }
    pub fn set_street_line_1(&mut self, street_line1: String) -> &mut Self {
        self.street_line1 = Some(street_line1);
        self
    }
    pub fn get_street_line_1(&self) -> Option<&String> {
        self.street_line1.as_ref()
    }
    pub fn set_street_line_2(&mut self, line_2: String) -> &mut Self {
        self.line_2 = Some(line_2);
        self
    }
    pub fn get_street_line_2(&self) -> Option<&String> {
        self.line_2.as_ref()
    }
    pub fn set_status(&mut self, status: AddressStatus) -> &mut Self {
        self.status = Some(status);
        self
    }
    pub fn get_status(&self) -> Option<&AddressStatus> {
        self.status.as_ref()
    }
    pub fn set_is_primary(&mut self, is_primary: bool) -> &mut Self {
        self.is_primary = Some(is_primary);
        self
    }
    pub fn get_is_primary(&self) -> Option<&bool> {
        self.is_primary.as_ref()
    }
    pub fn build(&mut self) -> AddressRequest {
        self.clone()
    }
}