#[cfg(test)]

mod tests {
    use dotenv::dotenv;
    use xendit::{
        client::XenditClient,
        common::Currency,
        payout::{
            CreatePayoutRequest, DigitalPayoutChannelProperties, GetPayoutChannelsRequestParams,
            GetPayoutsRequestParams, PayoutClient,
        },
    };

    fn initialize_client() -> XenditClient {
        dotenv().ok();
        XenditClient::new(std::env::var("XENDIT_API_KEY").unwrap())
    }

    #[tokio::test]
    async fn test_create_payout() {
        // Create idempotency-key based current unix time
        let idempotency_key = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .to_string();
        let client = initialize_client();
        let request_body = CreatePayoutRequest::new(
            String::from("Dummy"),
            String::from("ID_BCA"),
            DigitalPayoutChannelProperties::new("1234567890".to_string(), "Airai Iofi".to_string()),
            150000.0,
            Currency::IDR,
        );
        assert!(match PayoutClient::new(&client)
            .create_payout(request_body, idempotency_key, None)
            .await
        {
            Ok(payout) => !payout.get_id().is_empty(),
            Err(e) => {
                println!("{}", e);
                false
            }
        })
    }

    #[tokio::test]
    async fn test_get_payouts() {
        let client = initialize_client();
        let params = GetPayoutsRequestParams::new(String::from("Dummy"));
        assert!(
            match PayoutClient::new(&client).get_payouts(params, None).await {
                Ok(payouts) => {
                    println!("Length {:?}", payouts.get_data().len());
                    true
                }
                Err(e) => {
                    println!("{}", e);
                    false
                }
            }
        )
    }

    #[tokio::test]
    async fn test_get_payout_by_id() {
        let client = initialize_client();

        // Get List of payouts
        let params = GetPayoutsRequestParams::new(String::from("Dummy"));
        let payouts = PayoutClient::new(&client)
            .get_payouts(params, None)
            .await
            .unwrap();

        // Get Detail
        assert!(match PayoutClient::new(&client)
            .get_payout_by_id(payouts.get_data()[0].get_id().to_string(), None)
            .await
        {
            Ok(payout) => !payout.get_id().is_empty(),
            Err(e) => {
                println!("{}", e);
                false
            }
        })
    }

    #[tokio::test]
    async fn test_get_payout_channels() {
        let client = initialize_client();
        let params = GetPayoutChannelsRequestParams::new();
        assert!(match PayoutClient::new(&client)
            .get_payout_channels(params, None)
            .await
        {
            Ok(payouts) => {
                println!("Channel length {:?}", payouts.len());
                println!("Channel name {:?}", payouts[0].get_channel_name());
                true
            }
            Err(e) => {
                println!("{}", e);
                false
            }
        })
    }
}
