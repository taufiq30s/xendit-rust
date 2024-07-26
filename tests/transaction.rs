#[cfg(test)]

mod tests {
    use dotenv::dotenv;
    use xendit::{balance_and_transaction::{ChannelCategories, GetAllTransactionsRequestParams, TransactionClient, TransactionStatuses, TransactionTypes}, client::XenditClient};

    #[tokio::test]
    async fn test_get_transaction_list() {
        dotenv().ok();
        let client = XenditClient::new(std::env::var("XENDIT_API_KEY").unwrap_or("".to_string()));
        assert!(match TransactionClient::new(&client)
            .get_all_transactions(
                GetAllTransactionsRequestParams::new()
                    .set_types(vec![TransactionTypes::Payment])
                    .set_statuses(vec![TransactionStatuses::Success])
                    .set_channel_categories(vec![
                        ChannelCategories::Ewallet,
                        ChannelCategories::RetailOutlet
                    ])
                    .set_limit(2)
                    .build(),
                None
            )
            .await
        {
            Ok(_) => true,
            Err(e) => {
                panic!("Error: {}", e);
            }
        })
    }

    #[tokio::test]
    async fn test_get_transaction() {
        dotenv().ok();
        let client = XenditClient::new(std::env::var("XENDIT_API_KEY").unwrap_or("".to_string()));
        match TransactionClient::new(&client)
            .get_transaction_by_id(
                String::from("txn_c0f597f6-0361-4cd2-8c79-e195d17cd0e0"),
                None,
            )
            .await
        {
            Ok(res) => assert_eq!(res.get_reference_id(), Some("order-id-1715068880")),
            Err(e) => panic!("Error: {}", e),
        }
    }
}
