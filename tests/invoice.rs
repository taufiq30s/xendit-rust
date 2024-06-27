#[cfg(test)]

mod test {
    use dotenv::dotenv;
    use xendit::{client::XenditClient, invoice::{InvoiceBody, InvoiceClient}};

    #[tokio::test]
    async fn test_create() {
        dotenv().ok();
        let client = XenditClient::new(std::env::var("XENDIT_API_KEY").unwrap_or("".to_string()));
        let request_body = InvoiceBody::new(String::from("63b7c72b9c6818001035db03"), 510000.0);
        assert!(
            match InvoiceClient::new(&client).create(request_body).await {
                Ok(_) => true,
                Err(e) => panic!("Error: {}", e)
            }
        )
    }

    #[tokio::test]
    async fn test_get() {
        dotenv().ok();
        let client = XenditClient::new(std::env::var("XENDIT_API_KEY").unwrap_or("".to_string()));
        let invoice_id = String::from("667a94080d1dda6307187718");
        assert!(
            match InvoiceClient::new(&client).get(invoice_id).await {
                Ok(_) => true,
                Err(e) => panic!("Error: {}", e)
            }
        )
    }

    #[tokio::test]
    async fn test_list() {
        dotenv().ok();
        let client = XenditClient::new(std::env::var("XENDIT_API_KEY").unwrap_or("".to_string()));
        assert!(
            match InvoiceClient::new(&client).list().await {
                Ok(_) => true,
                Err(e) => panic!("Error: {}", e)
            }
        )
    }
}