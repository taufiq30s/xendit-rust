#[cfg(test)]

mod test {
    use dotenv::dotenv;
    use xendit::{client::XenditClient, invoice::{CreateInvoiceRequest, InvoiceClient}};

    #[tokio::test]
    async fn test_create_invoice() {
        dotenv().ok();
        let client = XenditClient::new(std::env::var("XENDIT_API_KEY").unwrap_or("".to_string()));
        let request_body = CreateInvoiceRequest::new(String::from("63b7c72b9c6818001035db04"), 510000.0);
        assert!(
            match InvoiceClient::new(&client).create_invoice(request_body, None).await {
                Ok(_) => true,
                Err(e) => panic!("Error: {}", e)
            }
        )
    }

    #[tokio::test]
    async fn test_get_invoice() {
        dotenv().ok();
        let client = XenditClient::new(std::env::var("XENDIT_API_KEY").unwrap_or("".to_string()));
        let request_body = CreateInvoiceRequest::new(String::from("63b7c72b9c6818001035db03"), 510000.0);
        let result = InvoiceClient::new(&client).create_invoice(request_body, None).await.expect("Failed Create Invoice");

        let invoice_id = result.get_id();
        assert!(
            match InvoiceClient::new(&client).get_invoice_by_id(invoice_id.to_string(), None).await {
                Ok(_) => true,
                Err(e) => panic!("Error: {}", e)
            }
        )
    }

    #[tokio::test]
    async fn test_get_list_of_invoices() {
        dotenv().ok();
        let client = XenditClient::new(std::env::var("XENDIT_API_KEY").unwrap_or("".to_string()));
        assert!(
            match InvoiceClient::new(&client).get_invoices(None).await {
                Ok(_) => true,
                Err(e) => panic!("Error: {}", e)
            }
        )
    }
}