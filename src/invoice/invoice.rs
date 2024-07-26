use reqwest::header::HeaderMap;

use crate::client::XenditClient;

use super::{CreateInvoiceRequest, Invoice};

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

    pub async fn create_invoice(
        &self,
        body: CreateInvoiceRequest,
        for_user_id: Option<String>,
    ) -> Result<Invoice, Box<dyn std::error::Error>> {
        let res = self
            .client
            .post::<Invoice, CreateInvoiceRequest>(
                "/v2/invoices",
                &body,
                self.process_custom_header(for_user_id).as_ref(),
            )
            .await?;
        Ok(res)
    }

    pub async fn get_invoice_by_id(
        &self,
        id: String,
        for_user_id: Option<String>,
    ) -> Result<Invoice, Box<dyn std::error::Error>> {
        let res = self
            .client
            .get::<Invoice>(
                &format!("/v2/invoices/{}", id),
                self.process_custom_header(for_user_id).as_ref(),
            )
            .await?;
        Ok(res)
    }

    pub async fn expire_invoice(
        &self,
        id: String,
        for_user_id: Option<String>,
    ) -> Result<Invoice, Box<dyn std::error::Error>> {
        let res = self
            .client
            .post::<Invoice, ()>(
                &format!("/invoices/{}/expire!", id),
                &(),
                self.process_custom_header(for_user_id).as_ref(),
            )
            .await?;
        Ok(res)
    }

    pub async fn get_invoices(
        &self,
        for_user_id: Option<String>,
    ) -> Result<Vec<Invoice>, Box<dyn std::error::Error>> {
        let res = self
            .client
            .get::<Vec<Invoice>>(
                "/v2/invoices",
                self.process_custom_header(for_user_id).as_ref(),
            )
            .await?;
        Ok(res)
    }
}
