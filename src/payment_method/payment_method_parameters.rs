use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::common::currency::Currency;

use super::PaymentChannel::{DirectDebit, EWallet, OverTheCounter, QRCode, VirtualAccount};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PaymentType {
    Card,
    Cryptocurrency,
    DirectBankTransfer,
    DirectDebit,
    Ewallet,
    OverTheCounter,
    QRCode,
    VirtualAccount,
    XenditEnumDefaultFallback
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PaymentCountry {
    Ph,
    Id,
    Vn,
    Th,
    My,
    Us
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Reusability {
    MultipleUse,
    OneTimeUse,
    XenditEnumDefaultFallback
}

// Card Method Parameters
#[derive(Serialize, Deserialize, Clone)]
pub struct CardChannelProperties {
    pub skip_three_d_secure: Option<bool>,
    pub success_return_url: Option<String>,
    pub failure_return_url: Option<String>,
    pub cardonfile_type: Option<String>,
    pub expires_at: Option<DateTime<Utc>>
}
#[derive(Serialize, Deserialize, Clone)]
pub struct CardInformation {
    pub card_number: String,
    pub expiry_month: String,
    pub expiry_year: String,
    pub cardholder_name: Option<String>,
    pub cvv: Option<String>
}
#[derive(Serialize, Deserialize, Clone)]
pub struct CardParameter {
    pub currency: Currency,
    pub channel_properties: CardChannelProperties,
    pub card_information: CardInformation
}
impl CardParameter {
    pub fn new(
        currency: Currency,
        card_number: String,
        expiry_month: String,
        expiry_year: String
    ) -> Self {
        Self {
            currency,
            channel_properties: CardChannelProperties {
                skip_three_d_secure: None,
                success_return_url: None,
                failure_return_url: None,
                cardonfile_type: None,
                expires_at: None
            },
            card_information: CardInformation {
                card_number,
                expiry_month,
                expiry_year,
                cardholder_name: None,
                cvv: None
            }
        }
    }
    pub fn set_skip_three_d_secure(&mut self, skip_three_d_secure: bool) -> &mut Self {
        self.channel_properties.skip_three_d_secure = Some(skip_three_d_secure);
        self
    }
    pub fn set_success_return_url(&mut self, success_return_url: String) -> &mut Self {
        self.channel_properties.success_return_url = Some(success_return_url);
        self
    }
    pub fn set_failure_return_url(&mut self, failure_return_url: String) -> &mut Self {
        self.channel_properties.failure_return_url = Some(failure_return_url);
        self
    }
    pub fn set_cardonfile_type(&mut self, cardonfile_type: String) -> &mut Self {
        self.channel_properties.cardonfile_type = Some(cardonfile_type);
        self
    }
    pub fn set_expires_at(&mut self, expires_at: DateTime<Utc>) -> &mut Self {
        self.channel_properties.expires_at = Some(expires_at);
        self
    }
    pub fn set_cardholder_name(&mut self, cardholder_name: String) -> &mut Self {
        self.card_information.cardholder_name = Some(cardholder_name);
        self
    }
    pub fn build(&mut self) -> Self {
        self.clone()
    }
}

// Direct Debit
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone)]
pub struct DirectDebitChannelProperties {
    pub success_return_url: Option<String>,
    pub failure_return_url: Option<String>,
    pub mobile_number: Option<String>,
    pub card_last_four: Option<String>,
    pub card_expiry: Option<String>,
    pub email: Option<String>,
    pub identity_document_number: Option<String>,
    pub require_auth: Option<bool>,
    pub account_number: Option<String>
}
#[derive(Serialize, Deserialize, Clone)]
pub struct DirectDebitParameter {
    pub channel_code: DirectDebit,
    pub channel_properties: DirectDebitChannelProperties
}
impl DirectDebitParameter {
    pub fn new(
        channel_code: DirectDebit
    ) -> Self {
        Self {
            channel_code,
            channel_properties: DirectDebitChannelProperties {
                success_return_url: None,
                failure_return_url: None,
                mobile_number: None,
                card_last_four: None,
                card_expiry: None,
                email: None,
                identity_document_number: None,
                require_auth: None,
                account_number: None
            }
        }
    }
    pub fn set_success_return_url(&mut self, success_return_url: String) -> &mut Self {
        self.channel_properties.success_return_url = Some(success_return_url);
        self
    }
    pub fn set_failure_return_url(&mut self, failure_return_url: String) -> &mut Self {
        self.channel_properties.failure_return_url = Some(failure_return_url);
        self
    }
    pub fn set_mobile_number(&mut self, mobile_number: String) -> &mut Self {
        self.channel_properties.mobile_number = Some(mobile_number);
        self
    }
    pub fn set_card_last_four(&mut self, card_last_four: String) -> &mut Self {
        self.channel_properties.card_last_four = Some(card_last_four);
        self
    }
    pub fn set_card_expiry(&mut self, card_expiry: String) -> &mut Self {
        self.channel_properties.card_expiry = Some(card_expiry);
        self
    }
    pub fn set_email(&mut self, email: String) -> &mut Self {
        self.channel_properties.email = Some(email);
        self
    }
    pub fn set_identity_document_number(&mut self, identity_document_number: String) -> &mut Self {
        self.channel_properties.identity_document_number = Some(identity_document_number);
        self
    }
    pub fn set_require_auth(&mut self, require_auth: bool) -> &mut Self {
        self.channel_properties.require_auth = Some(require_auth);
        self
    }
    pub fn set_account_number(&mut self, account_number: String) -> &mut Self {
        self.channel_properties.account_number = Some(account_number);
        self
    }
    pub fn build(&mut self) -> Self {
        self.clone()
    }
}

// EWallet
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone)]
pub struct EWalletChannelProperties {
    pub success_return_url: Option<String>,
    pub failure_return_url: Option<String>,
    pub cancel_return_url: Option<String>,
    pub pending_return_url: Option<String>,
    pub mobile_number: Option<String>,
    pub redeem_points: Option<String>,
    pub cashtag: Option<String>
}
#[derive(Serialize, Deserialize, Clone)]
pub struct EWalletAccount {
    pub name: Option<String>,
    pub account_details: Option<String>,
    pub balance: Option<f64>,
    pub point_balance: Option<f64>
}
#[derive(Serialize, Deserialize, Clone)]
pub struct EWalletParameter {
    pub channel_code: EWallet,
    pub channel_properties: EWalletChannelProperties,
    pub account: EWalletAccount
}
impl EWalletParameter {
    pub fn new(
        channel_code: EWallet
    ) -> Self {
        Self {
            channel_code,
            channel_properties: EWalletChannelProperties {
                success_return_url: None,
                failure_return_url: None,
                cancel_return_url: None,
                pending_return_url: None,
                mobile_number: None,
                redeem_points: None,
                cashtag: None
            },
            account: EWalletAccount {
                name: None,
                account_details: None,
                balance: None,
                point_balance: None
            }
        }
    }
    pub fn set_success_return_url(&mut self, success_return_url: String) -> &mut Self {
        self.channel_properties.success_return_url = Some(success_return_url);
        self
    }
    pub fn set_failure_return_url(&mut self, failure_return_url: String) -> &mut Self {
        self.channel_properties.failure_return_url = Some(failure_return_url);
        self
    }
    pub fn set_cancel_return_url(&mut self, cancel_return_url: String) -> &mut Self {
        self.channel_properties.cancel_return_url = Some(cancel_return_url);
        self
    }
    pub fn set_pending_return_url(&mut self, pending_return_url: String) -> &mut Self {
        self.channel_properties.pending_return_url = Some(pending_return_url);
        self
    }
    pub fn set_mobile_number(&mut self, mobile_number: String) -> &mut Self {
        self.channel_properties.mobile_number = Some(mobile_number);
        self
    }
    pub fn set_redeem_points(&mut self, redeem_points: String) -> &mut Self {
        self.channel_properties.redeem_points = Some(redeem_points);
        self
    }
    pub fn set_cashtag(&mut self, cashtag: String) -> &mut Self {
        self.channel_properties.cashtag = Some(cashtag);
        self
    }
    pub fn set_account_name(&mut self, name: String) -> &mut Self {
        self.account.name = Some(name);
        self
    }
    pub fn set_account_details(&mut self, account_details: String) -> &mut Self {
        self.account.account_details = Some(account_details);
        self
    }
    pub fn set_account_balance(&mut self, balance: f64) -> &mut Self {
        self.account.balance = Some(balance);
        self
    }
    pub fn set_account_point_balance(&mut self, point_balance: f64) -> &mut Self {
        self.account.point_balance = Some(point_balance);
        self
    }
    pub fn build(&mut self) -> Self {
        self.clone()
    }
}

// Over The Counter
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone)]
pub struct OverTheCounterChannelProperties {
    pub customer_name: String,
    pub payment_code: Option<String>,
    pub expires_at: Option<DateTime<Utc>>
}
#[derive(Serialize, Deserialize, Clone)]
pub struct OverTheCounterParameter {
    pub amount: Option<f64>,
    pub currency: Currency,
    pub channel_code: OverTheCounter,
    pub channel_properties: OverTheCounterChannelProperties
}
impl OverTheCounterParameter {
    pub fn new(
        channel_code: OverTheCounter
    ) -> Self {
        Self {
            amount: None,
            currency: Currency::IDR,
            channel_code,
            channel_properties: OverTheCounterChannelProperties {
                customer_name: String::new(),
                payment_code: None,
                expires_at: None
            }
        }
    }
    pub fn set_customer_name(&mut self, customer_name: String) -> &mut Self {
        self.channel_properties.customer_name = customer_name;
        self
    }
    pub fn set_payment_code(&mut self, payment_code: String) -> &mut Self {
        self.channel_properties.payment_code = Some(payment_code);
        self
    }
    pub fn set_expires_at(&mut self, expires_at: DateTime<Utc>) -> &mut Self {
        self.channel_properties.expires_at = Some(expires_at);
        self
    }
    pub fn build(&mut self) -> Self {
        self.clone()
    }
}
#[derive(Serialize, Clone)]
pub struct OverTheCounterParameterUpdate {
    pub amount: Option<f64>,
    pub channel_properties: Option<OverTheCounterChannelProperties>
}
impl OverTheCounterParameterUpdate {
    pub fn new() -> Self {
        Self {
            amount: None,
            channel_properties: None
        }
    }
    pub fn set_amount(&mut self, amount: f64) -> &mut Self {
        self.amount = Some(amount);
        self
    }
    pub fn set_customer_name(&mut self, customer_name: String) -> &mut Self {
        if let Some(channel_properties) = &mut self.channel_properties {
            channel_properties.customer_name = customer_name;
        }
        self
    }
    pub fn set_expires_at(&mut self, expires_at: DateTime<Utc>) -> &mut Self {
        if let Some(channel_properties) = &mut self.channel_properties {
            channel_properties.expires_at = Some(expires_at);
        }
        self
    }
    pub fn build(&mut self) -> Self {
        self.clone()
    }
}

// Virtual Account
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone)]
pub struct VirtualAccountChannelProperties {
    pub customer_name: Option<String>,
    pub virtual_account_number: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
    pub suggested_amount: Option<f64>
}
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone)]
pub struct VirtualAccountParameter {
    pub channel_code: VirtualAccount,
    pub channel_properties: VirtualAccountChannelProperties,
    pub amount: Option<f64>,
    pub min_amount: Option<f64>,
    pub max_amount: Option<f64>,
    pub currency: Option<Currency>,
    pub alternative_display_types: Option<Vec<String>>
}
impl VirtualAccountParameter {
    pub fn new(
        channel_code: VirtualAccount
    ) -> Self {
        Self {
            channel_code,
            channel_properties: VirtualAccountChannelProperties {
                customer_name: None,
                virtual_account_number: None,
                expires_at: None,
                suggested_amount: None
            },
            amount: None,
            min_amount: None,
            max_amount: None,
            currency: Some(Currency::IDR),
            alternative_display_types: None
        }
    }
    pub fn set_customer_name(&mut self, customer_name: String) -> &mut Self {
        self.channel_properties.customer_name = Some(customer_name);
        self
    }
    pub fn set_virtual_account_number(&mut self, virtual_account_number: String) -> &mut Self {
        self.channel_properties.virtual_account_number = Some(virtual_account_number);
        self
    }
    pub fn set_expires_at(&mut self, expires_at: DateTime<Utc>) -> &mut Self {
        self.channel_properties.expires_at = Some(expires_at);
        self
    }
    pub fn set_suggested_amount(&mut self, suggested_amount: f64) -> &mut Self {
        self.channel_properties.suggested_amount = Some(suggested_amount);
        self
    }
    pub fn set_amount(&mut self, amount: f64) -> &mut Self {
        self.amount = Some(amount);
        self
    }
    pub fn set_min_amount(&mut self, min_amount: f64) -> &mut Self {
        self.min_amount = Some(min_amount);
        self
    }
    pub fn set_max_amount(&mut self, max_amount: f64) -> &mut Self {
        self.max_amount = Some(max_amount);
        self
    }
    pub fn set_currency(&mut self, currency: Currency) -> &mut Self {
        self.currency = Some(currency);
        self
    }
    pub fn add_alternative_display_type(&mut self, alternative_display_type: String) -> &mut Self {
        if let Some(alternative_display_types) = &mut self.alternative_display_types {
            alternative_display_types.push(alternative_display_type);
        }
        self
    }
    pub fn build(&mut self) -> Self {
        self.clone()
    }
}
#[skip_serializing_none]
#[derive(Serialize, Clone)]
pub struct VirtualAccountParameterUpdate {
    pub amount: Option<f64>,
    pub min_amount: Option<f64>,
    pub max_amount: Option<f64>,
    pub channel_properties: Option<VirtualAccountChannelProperties>,
    pub alternative_display_types: Option<Vec<String>>
}
impl VirtualAccountParameterUpdate {
    pub fn new() -> Self {
        Self {
            amount: None,
            min_amount: None,
            max_amount: None,
            channel_properties: None,
            alternative_display_types: None
        }
    }
    pub fn set_amount(&mut self, amount: f64) -> &mut Self {
        self.amount = Some(amount);
        self
    }
    pub fn set_min_amount(&mut self, min_amount: f64) -> &mut Self {
        self.min_amount = Some(min_amount);
        self
    }
    pub fn set_max_amount(&mut self, max_amount: f64) -> &mut Self {
        self.max_amount = Some(max_amount);
        self
    }
    pub fn set_customer_name(&mut self, customer_name: String) -> &mut Self {
        if let Some(channel_properties) = &mut self.channel_properties {
            channel_properties.customer_name = Some(customer_name);
        }
        self
    }
    pub fn set_virtual_account_number(&mut self, virtual_account_number: String) -> &mut Self {
        if let Some(channel_properties) = &mut self.channel_properties {
            channel_properties.virtual_account_number = Some(virtual_account_number);
        }
        self
    }
    pub fn set_expires_at(&mut self, expires_at: DateTime<Utc>) -> &mut Self {
        if let Some(channel_properties) = &mut self.channel_properties {
            channel_properties.expires_at = Some(expires_at);
        }
        self
    }
    pub fn set_suggested_amount(&mut self, suggested_amount: f64) -> &mut Self {
        if let Some(channel_properties) = &mut self.channel_properties {
            channel_properties.suggested_amount = Some(suggested_amount);
        }
        self
    }
    pub fn add_alternative_display_type(&mut self, alternative_display_type: String) -> &mut Self {
        if let Some(alternative_display_types) = &mut self.alternative_display_types {
            alternative_display_types.push(alternative_display_type);
        }
        self
    }
    pub fn build(&mut self) -> Self {
        self.clone()
    }
}


// QRCode
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone)]
pub struct QRCodeChannelProperties {
    pub qr_string: Option<String>,
    pub expires_at: Option<DateTime<Utc>>
}#[derive(Serialize, Deserialize, Clone)]
pub struct QRCodeParameter {
    pub amount: Option<f64>,
    pub currency: Option<Currency>,
    pub channel_code: Option<QRCode>,
    pub channel_properties: QRCodeChannelProperties
}
impl QRCodeParameter {
    pub fn new(
    ) -> Self {
        Self {
            amount: None,
            currency: None,
            channel_code: None,
            channel_properties: QRCodeChannelProperties {
                qr_string: None,
                expires_at: None
            }
        }
    }
    pub fn set_qr_string(&mut self, qr_string: String) -> &mut Self {
        self.channel_properties.qr_string = Some(qr_string);
        self
    }
    pub fn set_expires_at(&mut self, expires_at: DateTime<Utc>) -> &mut Self {
        self.channel_properties.expires_at = Some(expires_at);
        self
    }
    pub fn build(&mut self) -> Self {
        self.clone()
    }
}