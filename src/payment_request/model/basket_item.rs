use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::common::Currency;

#[derive(Serialize, Deserialize, Clone)]
pub struct BasketItem {
    name: String,
    category: String,
    currency: Currency,
    quantity: f64,
    price: f64,
    reference_id: Option<String>,
    description: Option<String>,
    r#type: Option<String>,
    sub_category: Option<String>,
    payer_charged_currency: Option<String>,
    payer_charged_price: Option<f64>,
    url: Option<String>,
    metadata: Option<HashMap<String, serde_json::Value>>,
}

impl BasketItem {
    pub fn new(
        name: String,
        category: String,
        currency: Currency,
        quantity: f64,
        price: f64,
    ) -> Self {
        BasketItem {
            name,
            category,
            currency,
            quantity,
            price,
            reference_id: None,
            description: None,
            r#type: None,
            sub_category: None,
            payer_charged_currency: None,
            payer_charged_price: None,
            url: None,
            metadata: None,
        }
    }
    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }
    pub fn set_category(&mut self, category: String) -> &mut Self {
        self.category = category;
        self
    }
    pub fn get_category(&self) -> &String {
        &self.category
    }
    pub fn set_currency(&mut self, currency: Currency) -> &mut Self {
        self.currency = currency;
        self
    }
    pub fn get_currency(&self) -> &Currency {
        &self.currency
    }
    pub fn set_quantity(&mut self, quantity: f64) -> &mut Self {
        self.quantity = quantity;
        self
    }
    pub fn get_quantity(&self) -> &f64 {
        &self.quantity
    }
    pub fn set_price(&mut self, price: f64) -> &mut Self {
        self.price = price;
        self
    }
    pub fn get_price(&self) -> &f64 {
        &self.price
    }
    pub fn set_reference_id(&mut self, reference_id: String) -> &mut Self {
        self.reference_id = Some(reference_id);
        self
    }
    pub fn get_reference_id(&self) -> Option<&String> {
        self.reference_id.as_ref()
    }
    pub fn set_description(&mut self, description: String) -> &mut Self {
        self.description = Some(description);
        self
    }
    pub fn get_description(&self) -> Option<&String> {
        self.description.as_ref()
    }

    pub fn set_type(&mut self, r#type: String) -> &mut Self {
        self.r#type = Some(r#type);
        self
    }
    pub fn get_type(&self) -> Option<&String> {
        self.r#type.as_ref()
    }
    pub fn set_sub_category(&mut self, sub_category: String) -> &mut Self {
        self.sub_category = Some(sub_category);
        self
    }
    pub fn get_sub_category(&self) -> Option<&String> {
        self.sub_category.as_ref()
    }
    pub fn set_payer_charged_currency(&mut self, payer_charged_currency: String) -> &mut Self {
        self.payer_charged_currency = Some(payer_charged_currency);
        self
    }
    pub fn get_payer_charged_currency(&self) -> Option<&String> {
        self.payer_charged_currency.as_ref()
    }
    pub fn set_payer_charged_price(&mut self, payer_charged_price: f64) -> &mut Self {
        self.payer_charged_price = Some(payer_charged_price);
        self
    }
    pub fn get_payer_charged_price(&self) -> Option<&f64> {
        self.payer_charged_price.as_ref()
    }
    pub fn set_url(&mut self, url: String) -> &mut Self {
        self.url = Some(url);
        self
    }
    pub fn get_url(&self) -> Option<&String> {
        self.url.as_ref()
    }
    pub fn set_metadata(&mut self, metadata: HashMap<String, serde_json::Value>) -> &mut Self {
        self.metadata = Some(metadata);
        self
    }
    pub fn get_metadata(&self) -> Option<&HashMap<String, serde_json::Value>> {
        self.metadata.as_ref()
    }
    pub fn build(&mut self) -> Self {
        self.clone()
    }
}
