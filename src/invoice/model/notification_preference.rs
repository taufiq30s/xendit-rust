use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NotificationChannel {
    Email,
    SMS,
    WhatsApp,
    Viber
}

#[derive(Serialize, Deserialize, Clone)]
pub struct NotificationPreference {
    pub invoice_created: Option<Vec<NotificationChannel>>,
    pub invoice_reminder: Option<Vec<NotificationChannel>>,
    pub invoice_paid: Option<Vec<NotificationChannel>>,
}
impl NotificationPreference {
    pub fn new() -> Self {
        NotificationPreference {
            invoice_created: None,
            invoice_reminder: None,
            invoice_paid: None,
        }
    }
    pub fn get_invoice_created(&self) -> Option<&Vec<NotificationChannel>> {
        self.invoice_created.as_ref()
    }
    pub fn set_invoice_created(&mut self, channels: Vec<NotificationChannel>) -> &mut Self {
        self.invoice_created = Some(channels);
        self
    }
    pub fn get_invoice_reminder(&self) -> Option<&Vec<NotificationChannel>> {
        self.invoice_reminder.as_ref()
    }
    pub fn set_invoice_reminder(&mut self, channels: Vec<NotificationChannel>) -> &mut Self {
        self.invoice_reminder = Some(channels);
        self
    }
    pub fn get_invoice_paid(&self) -> Option<&Vec<NotificationChannel>> {
        self.invoice_paid.as_ref()
    }
    pub fn set_invoice_paid(&mut self, channels: Vec<NotificationChannel>) -> &mut Self {
        self.invoice_paid = Some(channels);
        self
    }
    pub fn build(&self) -> Self {
        self.clone()
    }
}