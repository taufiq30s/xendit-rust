use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::common::Currency;

use super::{ChannelProperties, Customer, InvoiceFee, InvoiceItem, NotificationPreference};

#[skip_serializing_none]
#[derive(Serialize, Clone)]
pub struct CreateInvoiceRequest {
    external_id: String,
    amount: f64,
    description: Option<String>,
    customer: Option<Customer>,
    customer_notification_preference: Option<NotificationPreference>,
    invoice_duration: Option<u64>,
    success_redirect_url: Option<String>,
    failure_redirect_url: Option<String>,
    payment_methods: Option<Vec<String>>,
    currency: Option<Currency>,
    callback_virtual_account_id: Option<String>,
    mid_label: Option<String>,
    reminder_time_unit: Option<String>,
    reminder_time: Option<u32>,
    locale: Option<String>,
    items: Option<Vec<InvoiceItem>>,
    fees: Option<Vec<InvoiceFee>>,
    should_authenticate_credit_card: Option<bool>,
    channel_properties: Option<ChannelProperties>,
    metadata: Option<serde_json::Value>,
}
impl CreateInvoiceRequest {
    pub fn new(external_id: String, amount: f64) -> Self {
        Self {
            external_id,
            amount,
            description: None,
            customer: None,
            customer_notification_preference: None,
            invoice_duration: None,
            success_redirect_url: None,
            failure_redirect_url: None,
            payment_methods: None,
            currency: None,
            callback_virtual_account_id: None,
            mid_label: None,
            reminder_time_unit: None,
            reminder_time: None,
            locale: None,
            items: None,
            fees: None,
            should_authenticate_credit_card: None,
            channel_properties: None,
            metadata: None,
        }
    }
    pub fn get_external_id(&self) -> &str {
        &self.external_id
    }
    pub fn set_external_id(&mut self, external_id: String) -> &mut Self {
        self.external_id = external_id;
        self
    }
    pub fn get_amount(&self) -> f64 {
        self.amount
    }
    pub fn set_amount(&mut self, amount: f64) -> &mut Self {
        self.amount = amount;
        self
    }
    pub fn get_description(&self) -> Option<&str> {
        self.description.as_deref()
    }
    pub fn set_description(&mut self, description: String) -> &mut Self {
        self.description = Some(description);
        self
    }
    pub fn get_customer(&self) -> Option<&Customer> {
        self.customer.as_ref()
    }
    pub fn set_customer(&mut self, customer: Customer) -> &mut Self {
        self.customer = Some(customer);
        self
    }
    pub fn get_customer_notification_preference(&self) -> Option<&NotificationPreference> {
        self.customer_notification_preference.as_ref()
    }
    pub fn set_customer_notification_preference(
        &mut self,
        customer_notification_preference: NotificationPreference,
    ) -> &mut Self {
        self.customer_notification_preference = Some(customer_notification_preference);
        self
    }
    pub fn get_invoice_duration(&self) -> Option<u64> {
        self.invoice_duration
    }
    pub fn set_invoice_duration(&mut self, invoice_duration: u64) -> &mut Self {
        self.invoice_duration = Some(invoice_duration);
        self
    }
    pub fn get_success_redirect_url(&self) -> Option<&str> {
        self.success_redirect_url.as_deref()
    }
    pub fn set_success_redirect_url(&mut self, success_redirect_url: String) -> &mut Self {
        self.success_redirect_url = Some(success_redirect_url);
        self
    }
    pub fn get_failure_redirect_url(&self) -> Option<&str> {
        self.failure_redirect_url.as_deref()
    }
    pub fn set_failure_redirect_url(&mut self, failure_redirect_url: String) -> &mut Self {
        self.failure_redirect_url = Some(failure_redirect_url);
        self
    }
    pub fn get_payment_methods(&self) -> Option<&Vec<String>> {
        self.payment_methods.as_ref()
    }
    pub fn set_payment_methods(&mut self, payment_methods: Vec<String>) -> &mut Self {
        self.payment_methods = Some(payment_methods);
        self
    }
    pub fn get_currency(&self) -> Option<&Currency> {
        self.currency.as_ref()
    }
    pub fn set_currency(&mut self, currency: Currency) -> &mut Self {
        self.currency = Some(currency);
        self
    }
    pub fn get_callback_virtual_account_id(&self) -> Option<&str> {
        self.callback_virtual_account_id.as_deref()
    }
    pub fn set_callback_virtual_account_id(
        &mut self,
        callback_virtual_account_id: String,
    ) -> &mut Self {
        self.callback_virtual_account_id = Some(callback_virtual_account_id);
        self
    }
    pub fn get_mid_label(&self) -> Option<&str> {
        self.mid_label.as_deref()
    }
    pub fn set_mid_label(&mut self, mid_label: String) -> &mut Self {
        self.mid_label = Some(mid_label);
        self
    }
    pub fn get_reminder_time_unit(&self) -> Option<&str> {
        self.reminder_time_unit.as_deref()
    }
    pub fn set_reminder_time_unit(&mut self, reminder_time_unit: String) -> &mut Self {
        self.reminder_time_unit = Some(reminder_time_unit);
        self
    }
    pub fn get_reminder_time(&self) -> Option<u32> {
        self.reminder_time
    }
    pub fn set_reminder_time(&mut self, reminder_time: u32) -> &mut Self {
        self.reminder_time = Some(reminder_time);
        self
    }
    pub fn get_locale(&self) -> Option<&str> {
        self.locale.as_deref()
    }
    pub fn set_locale(&mut self, locale: String) -> &mut Self {
        self.locale = Some(locale);
        self
    }
    pub fn get_items(&self) -> Option<&Vec<InvoiceItem>> {
        self.items.as_ref()
    }
    pub fn set_items(&mut self, items: Vec<InvoiceItem>) -> &mut Self {
        self.items = Some(items);
        self
    }
    pub fn get_fees(&self) -> Option<&Vec<InvoiceFee>> {
        self.fees.as_ref()
    }
    pub fn set_fees(&mut self, fees: Vec<InvoiceFee>) -> &mut Self {
        self.fees = Some(fees);
        self
    }
    pub fn get_should_authenticate_credit_card(&self) -> Option<bool> {
        self.should_authenticate_credit_card
    }
    pub fn set_should_authenticate_credit_card(
        &mut self,
        should_authenticate_credit_card: bool,
    ) -> &mut Self {
        self.should_authenticate_credit_card = Some(should_authenticate_credit_card);
        self
    }
    pub fn get_channel_properties(&self) -> Option<&ChannelProperties> {
        self.channel_properties.as_ref()
    }
    pub fn set_channel_properties(&mut self, channel_properties: ChannelProperties) -> &mut Self {
        self.channel_properties = Some(channel_properties);
        self
    }
    pub fn get_metadata(&self) -> Option<&serde_json::Value> {
        self.metadata.as_ref()
    }
    pub fn set_metadata(&mut self, metadata: serde_json::Value) -> &mut Self {
        self.metadata = Some(metadata);
        self
    }
    pub fn build(&self) -> Self {
        self.clone()
    }
}