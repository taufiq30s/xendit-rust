use serde::Deserialize;

use crate::payment_method::{DirectDebitChannel, DirectDebitChannelProperties};

#[derive(Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DirectDebitType {
    DebitCard,
    BankAccount,
    BankRedirect,
    Unknown
}

#[derive(Deserialize)]
pub struct DirectDebitBankAccount {
    masked_bank_account_number: Option<String>,
    bank_account_hash: Option<String>,
    mobile_number: Option<String>,
    identity_document_number: Option<String>
}
impl DirectDebitBankAccount {
    pub fn get_masked_bank_account_number(&self) -> Option<&str> {
        self.masked_bank_account_number.as_deref()
    }
    pub fn get_bank_account_hash(&self) -> Option<&str> {
        self.bank_account_hash.as_deref()
    }
    pub fn get_mobile_number(&self) -> Option<&str> {
        self.mobile_number.as_deref()
    }
    pub fn get_identity_document_number(&self) -> Option<&str> {
        self.identity_document_number.as_deref()
    }
}

#[derive(Deserialize)]
pub struct DirectDebitDebitCard {
    mobile_number: Option<String>,
    card_last_four: Option<String>,
    card_expiry: Option<String>,
    email: Option<String>,
    account_number: Option<String>
}
impl DirectDebitDebitCard {
    pub fn get_mobile_number(&self) -> Option<&str> {
        self.mobile_number.as_deref()
    }
    pub fn get_card_last_four(&self) -> Option<&str> {
        self.card_last_four.as_deref()
    }
    pub fn get_card_expiry(&self) -> Option<&str> {
        self.card_expiry.as_deref()
    }
    pub fn get_email(&self) -> Option<&str> {
        self.email.as_deref()
    }
    pub fn get_account_number(&self) -> Option<&str> {
        self.account_number.as_deref()
    }
}

#[derive(Deserialize)]
pub struct DirectDebit {
    channel_code: DirectDebitChannel,
    channel_properties: DirectDebitChannelProperties,
    r#type: DirectDebitType,
    bank_account: Option<DirectDebitBankAccount>,
    debit_card:Option<DirectDebitDebitCard>,
}
impl DirectDebit {
    pub fn get_channel_code(&self) -> &DirectDebitChannel {
        &self.channel_code
    }
    pub fn get_channel_properties(&self) -> &DirectDebitChannelProperties {
        &self.channel_properties
    }
    pub fn get_type(&self) -> &DirectDebitType {
        &self.r#type
    }
    pub fn get_bank_account(&self) -> Option<&DirectDebitBankAccount> {
        self.bank_account.as_ref()
    }
    pub fn get_debit_card(&self) -> Option<&DirectDebitDebitCard> {
        self.debit_card.as_ref()
    }
}