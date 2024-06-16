use serde::Serialize;

#[derive(Serialize)]
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
impl ToString for ChannelCategories {
    fn to_string(&self) -> String {
        match self {
            ChannelCategories::Cash => String::from("CASH"),
            ChannelCategories::Bank => String::from("BANK"),
            ChannelCategories::Cards => String::from("CARDS"),
            ChannelCategories::CardlessCredit => String::from("CARDLESS_CREDIT"),
            ChannelCategories::DirectDebit => String::from("DIRECT_DEBIT"),
            ChannelCategories::Ewallet => String::from("EWALLET"),
            ChannelCategories::Paylater => String::from("PAYLATER"),
            ChannelCategories::QrCode => String::from("QR_CODE"),
            ChannelCategories::RetailOutlet => String::from("RETAIL_OUTLET"),
            ChannelCategories::VirtualAccount => String::from("VIRTUAL_ACCOUNT"),
            ChannelCategories::XenPlatform => String::from("XEN_PLATFORM")
        }
    }
}