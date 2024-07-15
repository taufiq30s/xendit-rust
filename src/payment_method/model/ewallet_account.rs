use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone)]
pub struct EWalletAccount {
    name: Option<String>,
    account_details: Option<String>,
    balance: Option<f64>,
    point_balance: Option<f64>
}
impl EWalletAccount {
    pub(super) fn new() -> Self {
        Self {
            name: None,
            account_details: None,
            balance: None,
            point_balance: None
        }
    }
    pub fn get_name(&self) -> Option<&String> {
        self.name.as_ref()
    }
    pub fn get_account_details(&self) -> Option<&String> {
        self.account_details.as_ref()
    }
    pub fn get_balance(&self) -> Option<&f64> {
        self.balance.as_ref()
    }
    pub fn get_point_balance(&self) -> Option<&f64> {
        self.point_balance.as_ref()
    }
}