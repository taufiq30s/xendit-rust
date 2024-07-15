use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::payment_method::{EWalletAccount, EWalletChannel, EWalletChannelProperties};

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Clone)]
pub struct EWallet {
    channel_code: EWalletChannel,
    channel_properties: EWalletChannelProperties,
    account: EWalletAccount
}
impl EWallet {
    pub fn new(
        channel_code: EWalletChannel
    ) -> Self {
        Self {
            channel_code,
            channel_properties: EWalletChannelProperties::new(),
            account: EWalletAccount::new()
        }
    }
    pub fn get_channel_code(&self) -> &EWalletChannel {
        &self.channel_code
    }
    pub fn set_channel_code(&mut self, channel_code: EWalletChannel) -> &mut Self {
        self.channel_code = channel_code;
        self
    }
    pub fn get_channel_properties(&self) -> &EWalletChannelProperties {
        &self.channel_properties
    }
    pub fn set_channel_properties(&mut self, channel_properties: EWalletChannelProperties) -> &mut Self {
        self.channel_properties = channel_properties;
        self
    }
    pub fn get_account(&self) -> &EWalletAccount {
        &self.account
    }
    pub fn set_account(&mut self, account: EWalletAccount) -> &mut Self {
        self.account = account;
        self
    }
    pub fn build(&mut self) -> Self {
        self.clone()
    }
}