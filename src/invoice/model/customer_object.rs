use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Address {
    country: String,
    street_line1: Option<String>,
    street_line2: Option<String>,
    city: Option<String>,
    province_state: Option<String>,
    postal_code: Option<String>,
    category: Option<String>,
}
impl Address {
    pub fn new(coutry: String) -> Self {
        Address {
            country: coutry,
            street_line1: None,
            street_line2: None,
            city: None,
            province_state: None,
            postal_code: None,
            category: None,
        }
    }
    pub fn get_country(&self) -> &str {
        &self.country
    }
    pub fn set_country(&mut self, country: String) -> &mut Self {
        self.country = country;
        self
    }
    pub fn get_street_line1(&self) -> Option<&String> {
        self.street_line1.as_ref()
    }
    pub fn set_street_line1(&mut self, street_line1: String) -> &mut Self {
        self.street_line1 = Some(street_line1);
        self
    }
    pub fn get_street_line2(&self) -> Option<&String> {
        self.street_line2.as_ref()
    }
    pub fn set_street_line2(&mut self, street_line2: String) -> &mut Self {
        self.street_line2 = Some(street_line2);
        self
    }
    pub fn get_city(&self) -> Option<&String> {
        self.city.as_ref()
    }
    pub fn set_city(&mut self, city: String) -> &mut Self {
        self.city = Some(city);
        self
    }
    pub fn get_province_state(&self) -> Option<&String> {
        self.province_state.as_ref()
    }
    pub fn set_province_state(&mut self, province_state: String) -> &mut Self {
        self.province_state = Some(province_state);
        self
    }
    pub fn get_postal_code(&self) -> Option<&String> {
        self.postal_code.as_ref()
    }
    pub fn set_postal_code(&mut self, postal_code: String) -> &mut Self {
        self.postal_code = Some(postal_code);
        self
    }
    pub fn get_category(&self) -> Option<&String> {
        self.category.as_ref()
    }
    pub fn set_category(&mut self, category: String) -> &mut Self {
        self.category = Some(category);
        self
    }
    pub fn build(&self) -> Self {
        self.clone()
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Customer {
    given_name: Option<String>,
    surname: Option<String>,
    email: Option<String>,
    mobile_phone: Option<String>,
    address: Option<Address>,
}
impl Customer {
    pub fn new() -> Self {
        Customer {
            given_name: None,
            surname: None,
            email: None,
            mobile_phone: None,
            address: None,
        }
    }
    pub fn get_given_name(&self) -> Option<&String> {
        self.given_name.as_ref()
    }
    pub fn set_given_name(&mut self, given_name: String) -> &mut Self {
        self.given_name = Some(given_name);
        self
    }
    pub fn get_surname(&self) -> Option<&String> {
        self.surname.as_ref()
    }
    pub fn set_surname(&mut self, surname: String) -> &mut Self {
        self.surname = Some(surname);
        self
    }
    pub fn get_email(&self) -> Option<&String> {
        self.email.as_ref()
    }
    pub fn set_email(&mut self, email: String) -> &mut Self {
        self.email = Some(email);
        self
    }
    pub fn get_mobile_phone(&self) -> Option<&String> {
        self.mobile_phone.as_ref()
    }
    pub fn set_mobile_phone(&mut self, mobile_phone: String) -> &mut Self {
        self.mobile_phone = Some(mobile_phone);
        self
    }
    pub fn get_address(&self) -> Option<&Address> {
        self.address.as_ref()
    }
    pub fn set_address(&mut self, address: Address) -> &mut Self {
        self.address = Some(address);
        self
    }
    pub fn build(&self) -> Self {
        self.clone()
    }
}
