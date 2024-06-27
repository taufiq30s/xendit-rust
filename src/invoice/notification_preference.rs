use serde::{Deserialize, Serialize};

pub enum NotificationChannel {
    Email,
    SMS,
    WhatsApp,
    Viber
}
impl ToString for NotificationChannel {
    fn to_string(&self) -> String {
        match self {
            NotificationChannel::Email => "email".to_string(),
            NotificationChannel::SMS => "sms".to_string(),
            NotificationChannel::WhatsApp => "whatsapp".to_string(),
            NotificationChannel::Viber => "viber".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct NotificationPreference {
    pub invoice_created: Option<Vec<String>>,
    pub invoice_reminder: Option<Vec<String>>,
    pub invoice_paid: Option<Vec<String>>,
}
impl NotificationPreference {
    pub fn new() -> Self {
        NotificationPreference {
            invoice_created: None,
            invoice_reminder: None,
            invoice_paid: None,
        }
    }
    pub fn set_invoice_created(&mut self, channels: Vec<NotificationChannel>) -> &mut Self {
        self.invoice_created = Some(channels.iter().map(|c| c.to_string()).collect());
        self
    }
    pub fn set_invoice_reminder(&mut self, channels: Vec<NotificationChannel>) -> &mut Self {
        self.invoice_reminder = Some(channels.iter().map(|c| c.to_string()).collect());
        self
    }
    pub fn set_invoice_paid(&mut self, channels: Vec<NotificationChannel>) -> &mut Self {
        self.invoice_paid = Some(channels.iter().map(|c| c.to_string()).collect());
        self
    }
    pub fn build(&self) -> NotificationPreference {
        NotificationPreference {
            invoice_created: self.invoice_created.clone(),
            invoice_reminder: self.invoice_reminder.clone(),
            invoice_paid: self.invoice_paid.clone(),
        }
    }
}