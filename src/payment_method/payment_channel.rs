use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum EWallet {
    Dana,
    Ovo,
    Linkaja,
    Astrapay,
    Jeniuspay,
    Shopeepay,
    Grabpay,
    Gcash,
    Paymaya,
    Appota,
    Momo,
    Vnptwallet,
    Viettelpay,
    Zalopay,
    Wechatpay,
    Linepay,
    Truemoney,
    TouchnGo,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DirectDebit {
    Bri,
    Mandiri,
    Bpi,
    Rcbc,
    Unionbank,
    Chinabank,
    Scb,
    Bbl,
    Ktb,
    Bay,
    KbankMb,
    AffinFpx,
    AgroFpx,
    AllianceFpx,
    AmbankFpx,
    IslamFpx,
    MuamalatFpx,
    BocFpx,
    RakyatFpx,
    BsnFpx,
    CimbFpx,
    HlbFpx,
    HsbcFpx,
    KfhFpx,
    Mayb2eFpx,
    Mayb2uFpx,
    OcbcFpx,
    PublicFpx,
    RhbFpx,
    SchFpx,
    UobFpx,
    AffinFpxBusiness,
    AgroFpxBusiness,
    AllianceFpxBusiness,
    AmbankFpxBusiness,
    IslamFpxBusiness,
    MuamalatFpxBusiness,
    BnpFpxBusiness,
    CimbFpxBusiness,
    CitibankFpxBusiness,
    DeutscheFpxBusiness,
    HlbFpxBusiness,
    HsbcFpxBusiness,
    RakyatFpxBusiness,
    KfhFpxBusiness,
    Mayb2eFpxBusiness,
    OcbcFpxBusiness,
    PublicFpxBusiness,
    RhbFpxBusiness,
    SchFpxBusiness,
    UobFpxBusiness,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OverTheCounter {
    Alfamart,
    Indomaret,
    Sevenelevencodeconnect,
    SevenelevenCliqq,
    Cebuana,
    Ecpay,
    Palawan,
    Mlhuillier,
    EcpayDragonloan,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum VirtualAccount {
    Bca,
    Bsi,
    Bjb,
    Cimb,
    SahabatSampoerna,
    Artajasa,
    Bri,
    Bni,
    Mandiri,
    Permata,
    BankTransfer,
    Pv,
    Vietcapital,
    Woori,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum QRCode {
    Dana,
    Linkaja,
    Promptpay,
}
