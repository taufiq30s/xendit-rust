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