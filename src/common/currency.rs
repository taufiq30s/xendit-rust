//! Support Currencies
//! This enum based on Xendit Support Currencies's Documentation
//! 
//! See at: https://docs.xendit.co/credit-cards/supported-currencies

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Currency {
    IDR,
    USD,
    PHP,
    VND,
    THB,
    MYR
}