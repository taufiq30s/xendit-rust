use chrono::{DateTime, Utc};
use serde::Deserialize;

use super::{Bank, ChannelProperties, Customer, DirectDebit, Ewallet, InvoiceFee, InvoiceItem, NotificationPreference, Paylater, QRCode, RetailOutlet};

#[derive(Deserialize)]
pub struct Invoice {
    id: String,
    external_id: String,
    user_id: String,
    status: String,
    merchant_name: String,
    merchant_profile_picture_url: String,
    amount: f64,
    expiry_date: DateTime<Utc>,
    invoice_url: String,
    payer_email: Option<String>,
    description: Option<String>,
    customer: Option<Customer>,
    customer_notification_preference: Option<NotificationPreference>,
    available_banks: Vec<Bank>,
    available_retail_outlets: Vec<RetailOutlet>,
    available_ewallets: Vec<Ewallet>,
    available_qr_codes: Vec<QRCode>,
    available_direct_debits: Vec<DirectDebit>,
    available_paylaters: Vec<Paylater>,
    should_exclude_credit_card: bool,
    should_send_email: bool,
    created: DateTime<Utc>,
    updated: DateTime<Utc>,
    currency: String,
    items: Option<Vec<InvoiceItem>>,
    fees: Option<Vec<InvoiceFee>>,
    should_authenticate_credit_card: Option<bool>,
    channel_properties: Option<ChannelProperties>,
    metadata: Option<serde_json::Value>,
}
impl Invoice {
    pub fn get_id(&self) -> &str {
        &self.id
    }
    pub fn get_external_id(&self) -> &str {
        &self.external_id
    }
    pub fn get_user_id(&self) -> &str {
        &self.user_id
    }
    pub fn get_status(&self) -> &str {
        &self.status
    }
    pub fn get_merchant_name(&self) -> &str {
        &self.merchant_name
    }
    pub fn get_merchant_profile_picture_url(&self) -> &str {
        &self.merchant_profile_picture_url
    }
    pub fn get_amount(&self) -> f64 {
        self.amount
    }
    pub fn get_expiry_date(&self) -> &DateTime<Utc> {
        &self.expiry_date
    }
    pub fn get_invoice_url(&self) -> &str {
        &self.invoice_url
    }
    pub fn get_payer_email(&self) -> Option<&str> {
        self.payer_email.as_deref()
    }
    pub fn get_description(&self) -> Option<&str> {
        self.description.as_deref()
    }
    pub fn get_customer(&self) -> Option<&Customer> {
        self.customer.as_ref()
    }
    pub fn get_customer_notification_preference(&self) -> Option<&NotificationPreference> {
        self.customer_notification_preference.as_ref()
    }
    pub fn get_available_banks(&self) -> &Vec<Bank> {
        &self.available_banks
    }
    pub fn get_available_retail_outlets(&self) -> &Vec<RetailOutlet> {
        &self.available_retail_outlets
    }
    pub fn get_available_ewallets(&self) -> &Vec<Ewallet> {
        &self.available_ewallets
    }
    pub fn get_available_qr_codes(&self) -> &Vec<QRCode> {
        &self.available_qr_codes
    }
    pub fn get_available_direct_debits(&self) -> &Vec<DirectDebit> {
        &self.available_direct_debits
    }
    pub fn get_available_paylaters(&self) -> &Vec<Paylater> {
        &self.available_paylaters
    }
    pub fn should_exclude_credit_card(&self) -> bool {
        self.should_exclude_credit_card
    }
    pub fn should_send_email(&self) -> bool {
        self.should_send_email
    }
    pub fn get_created(&self) -> &DateTime<Utc> {
        &self.created
    }
    pub fn get_updated(&self) -> &DateTime<Utc> {
        &self.updated
    }
    pub fn get_currency(&self) -> &str {
        &self.currency
    }
    pub fn get_items(&self) -> Option<&Vec<InvoiceItem>> {
        self.items.as_ref()
    }
    pub fn get_fees(&self) -> Option<&Vec<InvoiceFee>> {
        self.fees.as_ref()
    }
    pub fn should_authenticate_credit_card(&self) -> Option<bool> {
        self.should_authenticate_credit_card
    }
    pub fn get_channel_properties(&self) -> Option<&ChannelProperties> {
        self.channel_properties.as_ref()
    }
    pub fn get_metadata(&self) -> Option<&serde_json::Value> {
        self.metadata.as_ref()
    }
}