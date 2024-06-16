//! Support Currencies
//! This enum based on Xendit Support Currencies's Documentation
//! 
//! See at: https://docs.xendit.co/credit-cards/supported-currencies

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone)]
pub enum Currency {
    IDR,
    USD,
    PHP,
    VND,
    THB,
    MYR
}
impl Currency {
    pub fn to_string(&self) -> String {
        match self {
            Currency::IDR => String::from("IDR"),
            Currency::USD => String::from("USD"),
            Currency::PHP => String::from("PHP"),
            Currency::VND => String::from("VND"),
            Currency::THB => String::from("THB"),
            Currency::MYR => String::from("MYR"),
        }
    }
}