use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

// Business Detail
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BusinessType {
    Corporation,
    SoleProprietorship,
    Partnership,
    Coorperative,
    Trust,
    NonProfit,
    Government
}

#[derive(Serialize, Deserialize, Clone)]
pub struct BusinessDetail {
    business_name: String,
    business_type: BusinessType,
    nature_of_business: Option<String>,
    business_domicile: Option<String>,
    date_of_registration: Option<NaiveDate>
}
impl BusinessDetail {
    pub fn new(business_name: String, business_type: BusinessType) -> Self {
        BusinessDetail {
            business_name,
            business_type,
            nature_of_business: None,
            business_domicile: None,
            date_of_registration: None
        }
    }
    pub fn set_business_name(&mut self, business_name: String) -> &mut Self {
        self.business_name = business_name;
        self
    }
    pub fn get_business_name(&self) -> &str {
        &self.business_name
    }
    pub fn set_business_type(&mut self, business_type: BusinessType) -> &mut Self {
        self.business_type = business_type;
        self
    }
    pub fn get_business_type(&self) -> &BusinessType {
        &self.business_type
    }
    pub fn set_nature_of_business(&mut self, nature_of_business: String) -> &mut Self {
        self.nature_of_business = Some(nature_of_business);
        self
    }
    pub fn get_nature_of_business(&self) -> Option<&String> {
        self.nature_of_business.as_ref()
    }
    pub fn set_business_domicile(&mut self, business_domicile: String) -> &mut Self {
        self.business_domicile = Some(business_domicile);
        self
    }
    pub fn get_business_domicile(&self) -> Option<&String> {
        self.business_domicile.as_ref()
    }
    pub fn set_date_of_registration(&mut self, date_of_registration: &str) -> &mut Self {
        self.date_of_registration = Some(NaiveDate::parse_from_str(date_of_registration, "%Y-%m-%d").unwrap());
        self
    }
    pub fn get_date_of_registration(&self) -> Option<&NaiveDate> {
        self.date_of_registration.as_ref()
    }
    pub fn build(&mut self) -> BusinessDetail {
        self.clone()
    }
}