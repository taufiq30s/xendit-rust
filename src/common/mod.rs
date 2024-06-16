pub mod date_filter;
pub mod currency;
pub mod transaction_status;
pub mod transaction_type;
pub mod transaction_channel;

pub trait ToString {
    fn to_string(&self) -> String;
}