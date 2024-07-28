#[cfg(test)]

mod tests {
    use dotenv::dotenv;
    use xendit::{
        client::XenditClient,
        common::Currency,
        payment_method::{
            EWallet, EWalletChannel, EWalletChannelProperties, PaymentMethodBody, PaymentType,
            Reusability, VirtualAccountChannel, VirtualAccountChannelProperties,
            VirtualAccountParameter,
        },
        payment_request::{PaymentRequestClient, PaymentRequestParameters},
    };

    fn initialize_client() -> XenditClient {
        dotenv().ok();
        XenditClient::new(std::env::var("XENDIT_API_KEY").unwrap())
    }

    #[tokio::test]
    async fn test_create_payment_request() {
        let client = initialize_client();
        let body = PaymentRequestParameters::new(Currency::IDR)
            .set_amount(150000.0)
            .set_payment_method(
                PaymentMethodBody::new(PaymentType::Ewallet, Reusability::OneTimeUse)
                    .set_ewallet(
                        EWallet::new(EWalletChannel::Shopeepay)
                            .set_channel_properties(
                                EWalletChannelProperties::new()
                                    .set_success_return_url(String::from(
                                        "https://your-redirect-website.com/success",
                                    ))
                                    .build(),
                            )
                            .build(),
                    )
                    .build(),
            )
            .build();

        assert!(match PaymentRequestClient::new(&client)
            .create_payment_request(body, None, None)
            .await
        {
            Ok(payment_request) => !payment_request.get_id().is_empty(),
            Err(e) => {
                println!("{}", e);
                false
            }
        })
    }

    #[tokio::test]
    async fn test_get_payment_request() {
        let client = initialize_client();

        // Create Payment Request First
        let body = PaymentRequestParameters::new(Currency::IDR)
            .set_amount(150000.0)
            .set_payment_method(
                PaymentMethodBody::new(PaymentType::VirtualAccount, Reusability::OneTimeUse)
                    .set_virtual_account(
                        VirtualAccountParameter::new(VirtualAccountChannel::Bri)
                            .set_channel_properties(
                                VirtualAccountChannelProperties::new()
                                    .set_customer_name(String::from("Airani Iofi"))
                                    .build(),
                            )
                            .build(),
                    )
                    .build(),
            )
            .set_customer_id(String::from("cust-b9a2f48e-bc3d-40b1-84b3-c8cb1c82349b"))
            .build();
        let payment_request = PaymentRequestClient::new(&client)
            .create_payment_request(body, None, None)
            .await
            .unwrap();

        assert!(match PaymentRequestClient::new(&client)
            .get_payment_request_by_id(payment_request.get_id().to_string(), None)
            .await
        {
            Ok(payment_request) => !payment_request.get_id().is_empty(),
            Err(e) => {
                println!("{}", e);
                false
            }
        })
    }

    #[tokio::test]
    async fn test_get_payment_request_list() {
        let client = initialize_client();

        assert!(
            match PaymentRequestClient::new(&client).get_all_payment_requests(None).await {
                Ok(payment_request_list) => !payment_request_list.get_data().is_empty(),
                Err(e) => {
                    println!("{}", e);
                    false
                }
            }
        )
    }
}
