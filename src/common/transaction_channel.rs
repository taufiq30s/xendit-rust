use serde::Serialize;

#[derive(Serialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ChannelCategories {
    Cash,
    Bank,
    Cards,
    CardlessCredit,
    DirectDebit,
    Ewallet,
    Paylater,
    QrCode,
    RetailOutlet,
    VirtualAccount,
    XenPlatform
}