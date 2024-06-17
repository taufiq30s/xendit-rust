#[cfg(test)]
mod tests {

    use dotenv::dotenv;
    use xendit::{balance_and_transaction::balance::{BalanceClient, GetBalanceParams}, client::XenditClient};

    #[tokio::test]
    async fn test_get_balance_default() {
        dotenv().ok();
        let client = XenditClient::new(std::env::var("XENDIT_API_KEY").unwrap_or("".to_string()));
        match BalanceClient::new(&client).get_balance(GetBalanceParams::new()).await {
            Ok(result) => {
                assert!(match result {
                    13991548668 | 13991549662 => true,
                    _ => false
                });
            },
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }

    #[tokio::test]
    async fn test_get_balance_with_holding_account_type() {
        dotenv().ok();
        let client = XenditClient::new(std::env::var("XENDIT_API_KEY").unwrap_or("".to_string()));
        match BalanceClient::new(&client).get_balance(
            GetBalanceParams::new()
                .set_account_type("HOLDING")
                .set_currency("IDR")
                .build()
        ).await {
            Ok(result) => {
                assert!(match result {
                    101000 | 14500000 => true,
                    _ => false
                });
            },
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}