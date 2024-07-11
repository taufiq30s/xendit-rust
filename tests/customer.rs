mod test {
    use chrono::Utc;
    use dotenv::dotenv;
    use xendit::{
        client::XenditClient,
        customer::{customer::CustomerClient, AddressCategory, AddressRequest, CustomerRequestBody, CustomerType, IndividualDetail, PatchCustomer},
    };

    fn initialize_client() -> XenditClient {
        dotenv().ok();
        XenditClient::new(std::env::var("XENDIT_API_KEY").unwrap())
    }

    #[tokio::test]
    async fn test_create_customer() {
        dotenv().ok();
        let client = initialize_client();
        let body = CustomerRequestBody::new(Utc::now().to_string(), CustomerType::Individual)
            .set_individual_detail(IndividualDetail::new(String::from("Airani")))
            .set_email(String::from("airani@example.com"))
            .set_mobile_number(String::from("+628567891234"))
            .set_addresses(vec![
                AddressRequest::new(
                    String::from("ID"))
                    .set_street_line_1(String::from("Jl. Merapi no. 10"))
                    .set_city(String::from("Jakarta"))
                    .set_postal_code(String::from("12345"))
                    .set_category(AddressCategory::Home)
                    .build()
            ])
            .build();
        assert!(
            match CustomerClient::new(&client).create(body, None, None).await {
                Ok(customer) => !customer.get_id().is_empty(),
                Err(e) => {
                    println!("{}", e);
                    false
                }
            }
        )
    }

    #[tokio::test]
    async fn test_get_customer() {
        dotenv().ok();
        let client = initialize_client();
        assert!(
            match CustomerClient::new(&client).get(String::from("cust-b9a2f48e-bc3d-40b1-84b3-c8cb1c82349b"), None).await {
                Ok(customer) => !customer.get_id().is_empty(),
                Err(e) => {
                    println!("{}", e);
                    false
                }
            }
        )
    }

    #[tokio::test]
    async fn test_get_customer_by_reference_id() {
        dotenv().ok();
        let client = initialize_client();
        assert!(
            match CustomerClient::new(&client).get_by_reference_id(String::from("2024-07-10 10:10:54.109704225 UTC"), None).await {
                Ok(customer) => !customer.get_data().is_empty(),
                Err(e) => {
                    println!("{}", e);
                    false
                }
            }
        )
    }

    #[tokio::test]
    async fn test_update_customer() {
        dotenv().ok();
        let client = initialize_client();
        let body = PatchCustomer::new()
            .set_description(String::from("Updated description"))
            .set_mobile_number(String::from("+628567891234"))
            .build();
        assert!(
            match CustomerClient::new(&client).update(String::from("cust-b9a2f48e-bc3d-40b1-84b3-c8cb1c82349b"), body, None).await {
                Ok(customer) => !customer.get_id().is_empty(),
                Err(e) => {
                    println!("{}", e);
                    false
                }
            }
        )
    }
}
