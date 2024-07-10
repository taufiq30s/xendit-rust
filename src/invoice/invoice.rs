use chrono::{DateTime, Utc};
use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{client::XenditClient, common::currency::Currency};

use super::{
    channel_object::Channel,
    customer_object::Customer,
    notification_preference::NotificationPreference,
    payment_object::{Bank, Ewallet, QRCode, RetailOutlet},
    DirectDebit, Paylater, PaymentDetail,
};

#[derive(Serialize, Deserialize, Clone)]
pub struct InvoiceItem {
    pub name: String,
    pub quantity: u64,
    pub price: f64,
    pub category: Option<String>,
    pub url: Option<String>,
}
impl InvoiceItem {
    pub fn new(name: String, quantity: u64, price: f64) -> Self {
        Self {
            name,
            quantity,
            price,
            category: None,
            url: None,
        }
    }
    pub fn set_category(&mut self, category: String) -> &mut Self {
        self.category = Some(category);
        self
    }
    pub fn set_url(&mut self, url: String) -> &mut Self {
        self.url = Some(url);
        self
    }
    pub fn build(&mut self) -> InvoiceItem {
        InvoiceItem {
            name: self.name.clone(),
            quantity: self.quantity.clone(),
            price: self.price.clone(),
            category: self.category.clone(),
            url: self.url.clone(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct InvoiceFee {
    pub r#type: String,
    pub value: f64,
}
impl InvoiceFee {
    pub fn new(r#type: String, value: f64) -> Self {
        Self { r#type, value }
    }
}

#[skip_serializing_none]
#[derive(Serialize)]
pub struct InvoiceBody {
    pub external_id: String,
    pub amount: f64,
    pub description: Option<String>,
    pub customer: Option<Customer>,
    pub customer_notification_preference: Option<NotificationPreference>,
    pub invoice_duration: Option<u64>,
    pub success_redirect_url: Option<String>,
    pub failure_redirect_url: Option<String>,
    pub payment_methods: Option<Vec<String>>,
    pub currency: Option<String>,
    pub callback_virtual_account_id: Option<String>,
    pub mid_label: Option<String>,
    pub reminder_time_unit: Option<String>,
    pub reminder_time: Option<u32>,
    pub locale: Option<String>,
    pub items: Option<Vec<InvoiceItem>>,
    pub fees: Option<Vec<InvoiceFee>>,
    pub should_authenticate_credit_card: Option<bool>,
    pub channel_properties: Option<Channel>,
    pub metadata: Option<serde_json::Value>,
}
impl InvoiceBody {
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
    pub fn set_description(&mut self, description: String) -> &mut Self {
        self.description = Some(description);
        self
    }
    pub fn set_customer(&mut self, customer: Customer) -> &mut Self {
        self.customer = Some(customer);
        self
    }
    pub fn set_customer_notification_preference(
        &mut self,
        customer_notification_preference: NotificationPreference,
    ) -> &mut Self {
        self.customer_notification_preference = Some(customer_notification_preference);
        self
    }
    pub fn set_invoice_duration(&mut self, invoice_duration: u64) -> &mut Self {
        self.invoice_duration = Some(invoice_duration);
        self
    }
    pub fn set_success_redirect_url(&mut self, success_redirect_url: String) -> &mut Self {
        self.success_redirect_url = Some(success_redirect_url);
        self
    }
    pub fn set_failure_redirect_url(&mut self, failure_redirect_url: String) -> &mut Self {
        self.failure_redirect_url = Some(failure_redirect_url);
        self
    }
    pub fn set_payment_methods(&mut self, payment_methods: Vec<String>) -> &mut Self {
        self.payment_methods = Some(payment_methods);
        self
    }
    pub fn set_currency(&mut self, currency: Currency) -> &mut Self {
        self.currency = Some(currency.to_string());
        self
    }
    pub fn set_callback_virtual_account_id(
        &mut self,
        callback_virtual_account_id: String,
    ) -> &mut Self {
        self.callback_virtual_account_id = Some(callback_virtual_account_id);
        self
    }
    pub fn set_mid_label(&mut self, mid_label: String) -> &mut Self {
        self.mid_label = Some(mid_label);
        self
    }
    pub fn set_reminder_time_unit(&mut self, reminder_time_unit: String) -> &mut Self {
        self.reminder_time_unit = Some(reminder_time_unit);
        self
    }
    pub fn set_reminder_time(&mut self, reminder_time: u32) -> &mut Self {
        self.reminder_time = Some(reminder_time);
        self
    }
    pub fn set_locale(&mut self, locale: String) -> &mut Self {
        self.locale = Some(locale);
        self
    }
    pub fn set_items(&mut self, items: Vec<InvoiceItem>) -> &mut Self {
        self.items = Some(items);
        self
    }
    pub fn set_fees(&mut self, fees: Vec<InvoiceFee>) -> &mut Self {
        self.fees = Some(fees);
        self
    }
    pub fn set_should_authenticate_credit_card(
        &mut self,
        should_authenticate_credit_card: bool,
    ) -> &mut Self {
        self.should_authenticate_credit_card = Some(should_authenticate_credit_card);
        self
    }
    pub fn set_channel_properties(&mut self, channel_properties: Channel) -> &mut Self {
        self.channel_properties = Some(channel_properties);
        self
    }
    pub fn set_metadata(&mut self, metadata: serde_json::Value) -> &mut Self {
        self.metadata = Some(metadata);
        self
    }
    pub fn build(&mut self) -> InvoiceBody {
        InvoiceBody {
            external_id: self.external_id.clone(),
            amount: self.amount.clone(),
            description: self.description.clone(),
            customer: self.customer.clone(),
            customer_notification_preference: self.customer_notification_preference.clone(),
            invoice_duration: self.invoice_duration.clone(),
            success_redirect_url: self.success_redirect_url.clone(),
            failure_redirect_url: self.failure_redirect_url.clone(),
            payment_methods: self.payment_methods.clone(),
            currency: self.currency.clone(),
            callback_virtual_account_id: self.callback_virtual_account_id.clone(),
            mid_label: self.mid_label.clone(),
            reminder_time_unit: self.reminder_time_unit.clone(),
            reminder_time: self.reminder_time.clone(),
            locale: self.locale.clone(),
            items: self.items.clone(),
            fees: self.fees.clone(),
            should_authenticate_credit_card: self.should_authenticate_credit_card.clone(),
            channel_properties: self.channel_properties.clone(),
            metadata: self.metadata.clone(),
        }
    }
}

#[derive(Deserialize)]
pub struct InvoiceResponse {
    pub id: String,
    pub external_id: String,
    pub user_id: String,
    pub status: String,
    pub merchant_name: String,
    pub merchant_profile_picture_url: String,
    pub amount: f64,
    pub expiry_date: DateTime<Utc>,
    pub invoice_url: String,
    pub payer_email: Option<String>,
    pub description: Option<String>,
    pub customer: Option<Customer>,
    pub customer_notification_preference: Option<NotificationPreference>,
    pub available_banks: Vec<Bank>,
    pub available_retail_outlets: Vec<RetailOutlet>,
    pub available_ewallets: Vec<Ewallet>,
    pub available_qr_codes: Vec<QRCode>,
    pub available_direct_debits: Vec<DirectDebit>,
    pub available_paylaters: Vec<Paylater>,
    pub should_exclude_credit_card: bool,
    pub should_send_email: bool,
    pub created: DateTime<Utc>,
    pub updated: DateTime<Utc>,
    pub currency: String,
    pub items: Option<Vec<InvoiceItem>>,
    pub fees: Option<Vec<InvoiceFee>>,
    pub should_authenticate_credit_card: Option<bool>,
    pub channel_properties: Option<Channel>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Deserialize)]
pub struct InvoiceCallback {
    pub id: String,
    pub external_id: String,
    pub user_id: String,
    pub is_high: bool,
    pub status: String,
    pub merchant_name: String,
    pub amount: f64,
    pub payer_email: Option<String>,
    pub description: Option<String>,
    pub paid_amount: Option<f64>,
    pub updated: DateTime<Utc>,
    pub created: DateTime<Utc>,
    pub currency: String,
    pub paid_at: Option<String>,
    pub payment_method: Option<String>,
    pub payment_channel: Option<String>,
    pub payment_destination: Option<String>,
    pub payment_details: Option<PaymentDetail>,
    pub payment_id: String,
    pub success_redirect_url: String,
    pub failure_redirect_url: String,
    pub credit_card_charge_id: Option<String>,
    pub items: Option<Vec<InvoiceItem>>,
    pub fees: Option<Vec<InvoiceFee>>,
    pub should_authenticate_credit_card: Option<bool>,
    pub bank_code: Option<String>,
    pub ewallet_type: Option<String>,
    pub on_demand_link: Option<String>,
    pub receipt_id: Option<String>,
}

pub struct InvoiceClient<'a> {
    client: &'a XenditClient,
}
impl<'a> InvoiceClient<'a> {
    pub fn new(client: &'a XenditClient) -> Self {
        Self { client }
    }
    fn process_custom_header(&self, for_user_id: Option<String>) -> Option<HeaderMap> {
        if for_user_id.is_none() {
            return None;
        }
        let mut headers = HeaderMap::new();
        headers.insert("for-user-id", for_user_id.unwrap().parse().unwrap());
        Some(headers)
    }
    pub async fn create(
        &self,
        body: InvoiceBody,
        for_user_id: Option<String>,
    ) -> Result<InvoiceResponse, Box<dyn std::error::Error>> {
        let res = self
            .client
            .post::<InvoiceResponse, InvoiceBody>(
                "/v2/invoices",
                &body,
                self.process_custom_header(for_user_id).as_ref(),
            )
            .await?;
        Ok(res)
    }
    pub async fn get(
        &self,
        id: String,
        for_user_id: Option<String>,
    ) -> Result<InvoiceResponse, Box<dyn std::error::Error>> {
        let res = self
            .client
            .get::<InvoiceResponse>(
                &format!("/v2/invoices/{}", id),
                self.process_custom_header(for_user_id).as_ref(),
            )
            .await?;
        Ok(res)
    }
    pub async fn expire(
        &self,
        id: String,
        for_user_id: Option<String>,
    ) -> Result<InvoiceResponse, Box<dyn std::error::Error>> {
        let res = self
            .client
            .post::<InvoiceResponse, ()>(
                &format!("/invoices/{}/expire!", id),
                &(),
                self.process_custom_header(for_user_id).as_ref(),
            )
            .await?;
        Ok(res)
    }
    pub async fn list(
        &self,
        for_user_id: Option<String>,
    ) -> Result<Vec<InvoiceResponse>, Box<dyn std::error::Error>> {
        let res = self
            .client
            .get::<Vec<InvoiceResponse>>(
                "/v2/invoices",
                self.process_custom_header(for_user_id).as_ref(),
            )
            .await?;
        Ok(res)
    }
}
