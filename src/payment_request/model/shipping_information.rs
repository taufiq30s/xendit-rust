use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone)]
pub struct ShippingInformation {
    country: String,
    city: Option<String>,
    street_line1: Option<String>,
    street_line2: Option<String>,
    province_state: Option<String>,
    postal_code: Option<String>,
}
impl ShippingInformation {
    pub fn new(country: String) -> Self {
        ShippingInformation {
            country,
            province_state: None,
            city: None,
            postal_code: None,
            street_line1: None,
            street_line2: None
        }
    }
    pub fn set_country(&mut self, country: String) -> &mut Self {
        self.country = country;
        self
    }
    pub fn get_country(&self) -> &String {
        &self.country
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
    pub fn set_street_line_2(&mut self, street_line2: String) -> &mut Self {
        self.street_line2 = Some(street_line2);
        self
    }
    pub fn get_street_line_2(&self) -> Option<&String> {
        self.street_line2.as_ref()
    }
    pub fn build(&mut self) -> Self {
        self.clone()
    }
}