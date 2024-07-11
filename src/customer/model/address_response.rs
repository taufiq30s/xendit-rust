use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::AddressStatus;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AddressCategory {
    Home,
    Work,
    Provincial
}

#[derive(Deserialize)]
pub struct Address {
    id: Option<String>,
    category: Option<AddressCategory>,
    country: String,
    province_state: Option<String>,
    city: Option<String>,
    postal_code: Option<String>,
    street_line1: Option<String>,
    street_line2: Option<String>,
    status: Option<AddressStatus>,
    is_primary: Option<bool>,
    meta: Option<HashMap<String, serde_json::Value>>,
    created: Option<DateTime<Utc>>,
    updated: Option<DateTime<Utc>>,
}
impl Address {
    pub fn get_id(&self) -> Option<&str> {
        self.id.as_deref()
    }
    pub fn get_category(&self) -> Option<&AddressCategory> {
        self.category.as_ref()
    }
    pub fn get_country(&self) -> &str {
        self.country.as_str()
    }
    pub fn get_province_state(&self) -> Option<&str> {
        self.province_state.as_deref()
    }
    pub fn get_city(&self) -> Option<&str> {
        self.city.as_deref()
    }
    pub fn get_postal_code(&self) -> Option<&str> {
        self.postal_code.as_deref()
    }
    pub fn get_street_line1(&self) -> Option<&str> {
        self.street_line1.as_deref()
    }
    pub fn get_street_line2(&self) -> Option<&str> {
        self.street_line2.as_deref()
    }
    pub fn get_status(&self) -> Option<&AddressStatus> {
        self.status.as_ref()
    }
    pub fn get_is_primary(&self) -> Option<bool> {
        self.is_primary
    }
    pub fn get_meta(&self) -> Option<&HashMap<String, serde_json::Value>> {
        self.meta.as_ref()
    }
    pub fn get_created(&self) -> Option<&DateTime<Utc>> {
        self.created.as_ref()
    }
    pub fn get_updated(&self) -> Option<&DateTime<Utc>> {
        self.updated.as_ref()
    }
}