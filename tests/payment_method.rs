#[cfg(test)]
mod tests {
    use chrono::Duration;
    use dotenv::dotenv;
    use tokio::time::sleep;
    use xendit::{
        client::XenditClient,
        payment_method::{
            payment_method::PaymentMethodClient, EWallet, EWalletChannel, EWalletChannelProperties,
            GetPaymentMethodsParams, PaymentMethodBody, PaymentMethodUpdateBody, PaymentStatus,
            PaymentType, Reusability, VirtualAccountChannel,
            VirtualAccountChannelProperties, VirtualAccountParameter,
            VirtualAccountParameterUpdate,
        },
    };

    fn initialize_client() -> XenditClient {
        dotenv().ok();
        XenditClient::new(std::env::var("XENDIT_API_KEY").unwrap())
    }

    #[tokio::test]
    async fn create_payment_method_ewallet() {
        let client = initialize_client();
        let body = PaymentMethodBody::new(PaymentType::Ewallet, Reusability::OneTimeUse)
            .set_description(String::from("Payment for order #123"))
            .set_ewallet(
                EWallet::new(EWalletChannel::Ovo)
                    .set_channel_properties(
                        EWalletChannelProperties::new()
                            .set_mobile_number(String::from("62817555493"))
                            .build(),
                    )
                    .build(),
            )
            .build();
        assert!(match PaymentMethodClient::new(&client)
            .create_payment_method(body, None)
            .await
        {
            Ok(payment_method) => !payment_method.get_id().is_empty(),
            Err(e) => {
                println!("{}", e);
                false
            }
        })
    }

    #[tokio::test]
    async fn create_payment_method_va() {
        let client = initialize_client();
        let body = PaymentMethodBody::new(PaymentType::VirtualAccount, Reusability::MultipleUse)
            .set_description(String::from("Payment for order #123"))
            .set_virtual_account(
                VirtualAccountParameter::new(VirtualAccountChannel::Bri)
                    .set_amount(120000.0)
                    .set_channel_properties(
                        VirtualAccountChannelProperties::new()
                            .set_customer_name(String::from("Airani Iofi"))
                            .set_expires_at(chrono::Utc::now() + Duration::days(2))
                            .build(),
                    )
                    .build(),
            )
            .build();
        assert!(match PaymentMethodClient::new(&client)
            .create_payment_method(body, None)
            .await
        {
            Ok(payment_method) => !payment_method.get_id().is_empty(),
            Err(e) => {
                println!("{}", e);
                false
            }
        })
    }

    #[tokio::test]
    async fn get_payment_method_by_id() {
        let client = initialize_client();
        // Create a payment method first
        let body = PaymentMethodBody::new(PaymentType::Ewallet, Reusability::OneTimeUse)
            .set_description(String::from("Payment for order #123"))
            .set_ewallet(
                EWallet::new(EWalletChannel::Ovo)
                    .set_channel_properties(
                        EWalletChannelProperties::new()
                            .set_mobile_number(String::from("62817555493"))
                            .build(),
                    )
                    .build(),
            )
            .build();
        let payment_method = PaymentMethodClient::new(&client)
            .create_payment_method(body, None)
            .await
            .unwrap();

        // Get Payment Method by ID
        assert!(match PaymentMethodClient::new(&client)
            .get_payment_method_by_id(&payment_method.get_id(), None)
            .await
        {
            Ok(payment_method) => !payment_method.get_id().is_empty(),
            Err(e) => {
                println!("{}", e);
                false
            }
        })
    }

    #[tokio::test]
    async fn update_payment_method() {
        let client = initialize_client();
        // Create a payment method first
        let body = PaymentMethodBody::new(PaymentType::VirtualAccount, Reusability::MultipleUse)
            .set_description(String::from("Payment for order #1507"))
            .set_virtual_account(
                VirtualAccountParameter::new(VirtualAccountChannel::Bri)
                    .set_amount(120000.0)
                    .set_channel_properties(
                        VirtualAccountChannelProperties::new()
                            .set_customer_name(String::from("Airani Iofi"))
                            .set_expires_at(chrono::Utc::now() + Duration::days(2))
                            .build(),
                    )
                    .build(),
            )
            .build();
        let payment_method = PaymentMethodClient::new(&client)
            .create_payment_method(body, None)
            .await
            .unwrap();
        sleep(std::time::Duration::from_millis(500)).await;

        // Update payment method
        let updated_body = PaymentMethodUpdateBody::new()
            .set_description(String::from("Updated Payment for order #1507"))
            .set_virtual_account(
                VirtualAccountParameterUpdate::new()
                    .set_amount(150000.0)
                    .build(),
            )
            .build();
        assert!(match PaymentMethodClient::new(&client)
            .patch_payment_method(&payment_method.get_id(), updated_body, None)
            .await
        {
            Ok(payment_method) => !payment_method.get_id().is_empty(),
            Err(e) => {
                println!("{}", e);
                false
            }
        })
    }

    #[tokio::test]
    async fn get_all_payment_methods() {
        let client = initialize_client();
        assert!(match PaymentMethodClient::new(&client)
            .get_all_payment_methods(None, None)
            .await
        {
            Ok(payment_methods) => !payment_methods.data.is_empty(),
            Err(e) => {
                println!("{}", e);
                false
            }
        })
    }

    #[tokio::test]
    async fn get_all_payment_methods_with_params() {
        let client = initialize_client();
        let params = GetPaymentMethodsParams::new()
            .set_status(vec![PaymentStatus::Pending, PaymentStatus::Active])
            .set_limit(5)
            .build();
        assert!(match PaymentMethodClient::new(&client)
            .get_all_payment_methods(Some(params), None)
            .await
        {
            Ok(payment_methods) => !payment_methods.data.is_empty(),
            Err(e) => {
                println!("{}", e);
                false
            }
        })
    }

    #[tokio::test]
    async fn set_payment_method_expire() {
        let client = initialize_client();
        // Create a payment method first
        let body = PaymentMethodBody::new(PaymentType::VirtualAccount, Reusability::MultipleUse)
            .set_description(String::from("Payment for order #1507"))
            .set_virtual_account(
                VirtualAccountParameter::new(VirtualAccountChannel::Bri)
                    .set_amount(120000.0)
                    .set_channel_properties(
                        VirtualAccountChannelProperties::new()
                            .set_customer_name(String::from("Airani Iofi"))
                            .set_expires_at(chrono::Utc::now() + Duration::days(2))
                            .build(),
                    )
                    .build(),
            )
            .build();
        let payment_method = PaymentMethodClient::new(&client)
            .create_payment_method(body, None)
            .await
            .unwrap();

        // Expire payment method
        assert!(match PaymentMethodClient::new(&client)
            .expire_payment_method(&payment_method.get_id(), None, None)
            .await
        {
            Ok(payment_method) => !payment_method.get_id().is_empty(),
            Err(e) => {
                println!("{}", e);
                false
            }
        })
    }

    #[tokio::test]
    async fn get_payments_by_payment_method_id() {
        let client = initialize_client();
        let payment_method_client = PaymentMethodClient::new(&client);
        // Create a payment method first
        let body = PaymentMethodBody::new(PaymentType::VirtualAccount, Reusability::OneTimeUse)
            .set_description(String::from("Payment for order #123"))
            .set_virtual_account(
                VirtualAccountParameter::new(VirtualAccountChannel::Bri)
                    .set_amount(120000.0)
                    .set_channel_properties(
                        VirtualAccountChannelProperties::new()
                            .set_customer_name(String::from("Airani Iofi"))
                            .set_expires_at(chrono::Utc::now() + Duration::days(2))
                            .build(),
                    )
                    .build(),
            )
            .build();
        let payment_method = payment_method_client
            .create_payment_method(body, None)
            .await
            .unwrap();
        sleep(std::time::Duration::from_millis(500)).await;

        // Simulate payment (set payment was success)
        let status = payment_method_client
            .simulate_payment(&payment_method.get_id(), 120000.0)
            .await
            .unwrap();
        println!("status {}; message {}", status.get_status(), status.get_message());
        sleep(std::time::Duration::from_secs(1)).await;

        // Execute
        assert!(match payment_method_client
            .get_payments_by_payment_method_id(
                "pm-e64c59c3-8f25-4e70-abc1-cc2d1973780d".to_string(),
                None,
                None
            )
            .await
        {
            Ok(payment) => !payment.data.is_empty(),
            Err(e) => {
                println!("{}", e);
                false
            }
        })
    }

    // TODO: Create Customer Client to use this unit test
    // #[tokio::test]
    // // This endpoint only applies to BRI Direct Debit.
    // async fn auth_otp() {
    //     let client = initialize_client();
    //     // Create a payment method first
    //     let body = PaymentMethodBody::new(PaymentType::DirectDebit, Reusability::OneTimeUse)
    //         .set_description(String::from("Payment for order #123"))
    //         .set_direct_debit(DirectDebitParameter::new(
    //             DirectDebit::Bri)
    //             .set_require_auth(true)
    //             .set_mobile_number(String::from("+62817555493"))
    //             .build()
    //         )
    //         .set_customer_id("cust-239c16f4-866d-43e8-9341-7badafbc019f".to_string()).build();
    //     let payment_method = PaymentMethodClient::new(&client).create_payment_method(body).await.unwrap();

    //     // Execute
    //     assert!(
    //         match PaymentMethodClient::new(&client).auth_payment_method(&payment_method.id, "123456").await {
    //             Ok(payment_method) => !payment_method.id.is_empty(),
    //             Err(e) => {
    //                 println!("{}", e);
    //                 false
    //             }
    //         }
    //     )
    // }
}
