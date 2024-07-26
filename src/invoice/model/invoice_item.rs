use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct InvoiceItem {
    name: String,
    quantity: u64,
    price: f64,
    category: Option<String>,
    url: Option<String>,
}
impl InvoiceItem {
    pub fn new(name: String, quantity: u64, price: f64) -> Self {
        Self {
            name,
            quantity,
            price,
            category: None,
            url: None,
        }
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.name = name;
        self
    }
    pub fn get_quantity(&self) -> u64 {
        self.quantity
    }
    pub fn set_quantity(&mut self, quantity: u64) -> &mut Self {
        self.quantity = quantity;
        self
    }
    pub fn get_price(&self) -> f64 {
        self.price
    }
    pub fn set_price(&mut self, price: f64) -> &mut Self {
        self.price = price;
        self
    }
    pub fn get_category(&self) -> Option<&str> {
        self.category.as_deref()
    }
    pub fn set_category(&mut self, category: String) -> &mut Self {
        self.category = Some(category);
        self
    }
    pub fn get_url(&self) -> Option<&str> {
        self.url.as_deref()
    }
    pub fn set_url(&mut self, url: String) -> &mut Self {
        self.url = Some(url);
        self
    }
    pub fn build(&mut self) -> Self {
        self.clone()
    }
}