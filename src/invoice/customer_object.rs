use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Customer {
    pub given_name: Option<String>,
    pub surname: Option<String>,
    pub email: Option<String>,
    pub mobile_phone: Option<String>,
    pub address: Option<Address>,
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
    pub fn set_given_name(&mut self, given_name: String) -> &mut Self {
        self.given_name = Some(given_name);
        self
    }
    pub fn set_surname(&mut self, surname: String) -> &mut Self {
        self.surname = Some(surname);
        self
    }
    pub fn set_email(&mut self, email: String) -> &mut Self {
        self.email = Some(email);
        self
    }
    pub fn set_mobile_phone(&mut self, mobile_phone: String) -> &mut Self {
        self.mobile_phone = Some(mobile_phone);
        self
    }
    pub fn set_address(&mut self, address: Address) -> &mut Self {
        self.address = Some(address);
        self
    }
    pub fn build(&self) -> Customer {
        Customer {
            given_name: self.given_name.clone(),
            surname: self.surname.clone(),
            email: self.email.clone(),
            mobile_phone: self.mobile_phone.clone(),
            address: self.address.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Address {
    pub country: String,
    pub street_line1: Option<String>,
    pub street_line2: Option<String>,
    pub city: Option<String>,
    pub province_state: Option<String>,
    pub postal_code: Option<String>,
    pub category: Option<String>,
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
    pub fn set_street_line1(&mut self, street_line1: String) -> &mut Self {
        self.street_line1 = Some(street_line1);
        self
    }
    pub fn set_street_line2(&mut self, street_line2: String) -> &mut Self {
        self.street_line2 = Some(street_line2);
        self
    }
    pub fn set_city(&mut self, city: String) -> &mut Self {
        self.city = Some(city);
        self
    }
    pub fn set_province_state(&mut self, province_state: String) -> &mut Self {
        self.province_state = Some(province_state);
        self
    }
    pub fn set_postal_code(&mut self, postal_code: String) -> &mut Self {
        self.postal_code = Some(postal_code);
        self
    }
    pub fn set_category(&mut self, category: String) -> &mut Self {
        self.category = Some(category);
        self
    }
    pub fn build(&self) -> Address {
        Address {
            country: self.country.clone(),
            street_line1: self.street_line1.clone(),
            street_line2: self.street_line2.clone(),
            city: self.city.clone(),
            province_state: self.province_state.clone(),
            postal_code: self.postal_code.clone(),
            category: self.category.clone(),
        }
    }
}