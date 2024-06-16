#[cfg(test)]

mod tests {
    use chrono::{DateTime, Utc};
    use dotenv::dotenv;
    use xendit::{balance_and_transaction::transaction::{TransactionClient, TransactionFee, TransactionLinks, TransactionListObject, TransactionListParams, TransactionObject}, client::XenditClient, common::{transaction_channel::ChannelCategories, transaction_status::TransactionStatus, transaction_type::TransactionType}};

    #[tokio::test]
    async fn test_get_transaction_list() {
        dotenv().ok();
        let client = XenditClient::new(std::env::var("XENDIT_API_KEY").unwrap_or("".to_string()));
        let expected = TransactionListObject {
            has_more: true,
            data: vec![
                TransactionObject {
                    id: String::from("txn_c0f597f6-0361-4cd2-8c79-e195d17cd0e0"),
                    product_id: String::from("5c27eea0-4963-41ba-994f-73a810a0b8ea"),
                    r#type: String::from("PAYMENT"),
                    channel_code: Some(String::from("ID_OVO")),
                    reference_id: Some(String::from("order-id-1715068880")),
                    account_identifier: Some(String::from("+6285156209625")),
                    currency: Some(String::from("IDR")),
                    amount: 10000,
                    net_amount: 9834,
                    cashflow: String::from("MONEY_IN"),
                    status: String::from("SUCCESS"),
                    channel_category: String::from("EWALLET"),
                    business_id: String::from("57fdbb445eec38910d3a4c47"),
                    created: "2024-05-07T08:01:21.640Z".parse::<DateTime<Utc>>().unwrap(),
                    updated: "2024-05-09T08:01:31.452Z".parse::<DateTime<Utc>>().unwrap(),
                    fee: TransactionFee{
                        xendit_fee: 150,
                        value_added_tax: 16,
                        xendit_withholding_tax: 0,
                        third_party_withholding_tax: 0,
                        status: String::from("COMPLETED")
                    },
                    settlement_status: Some(String::from("SETTLED")),
                    estimated_settlement_time: Some("2024-05-09T08:01:20.281Z".parse::<DateTime<Utc>>().unwrap())
                },
                TransactionObject {
                    id: String::from("txn_91b7354c-a99b-453b-a8cb-8cb00a84f7ea"),
                    product_id: String::from("cbe85141-df50-4098-a377-3b5ea25375cd"),
                    r#type: String::from("PAYMENT"),
                    channel_code: Some(String::from("ID_SHOPEEPAY")),
                    reference_id: Some(String::from("123")),
                    account_identifier: None,
                    currency: Some(String::from("IDR")),
                    amount: 10000000,
                    net_amount: 9850039,
                    cashflow: String::from("MONEY_IN"),
                    status: String::from("SUCCESS"),
                    channel_category: String::from("EWALLET"),
                    business_id: String::from("57fdbb445eec38910d3a4c47"),
                    created: "2023-07-28T04:36:58.568Z".parse::<DateTime<Utc>>().unwrap(),
                    updated: "2023-07-30T17:04:39.149Z".parse::<DateTime<Utc>>().unwrap(),
                    fee: TransactionFee{
                        xendit_fee: 135100,
                        value_added_tax: 14861,
                        xendit_withholding_tax: 0,
                        third_party_withholding_tax: 0,
                        status: String::from("COMPLETED")
                    },
                    settlement_status: Some(String::from("SETTLED")),
                    estimated_settlement_time: Some("2023-07-30T17:00:00.000Z".parse::<DateTime<Utc>>().unwrap())
                },
            ],
            links: Some(vec![TransactionLinks{
                href: String::from("/transactions?types%5B0%5D=PAYMENT&statuses%5B0%5D=SUCCESS&channel_categories%5B0%5D=EWALLET&channel_categories%5B1%5D=RETAIL_OUTLET&limit=2&after_id=txn_91b7354c-a99b-453b-a8cb-8cb00a84f7ea"),
                method: String::from("GET"),
                rel: String::from("next")
            }])
        };
        match TransactionClient::new(&client)
        .list(
            TransactionListParams::new()
            .set_types(vec![TransactionType::Payment])
            .set_statuses(vec![TransactionStatus::Success])
            .set_channel_categories(vec![ChannelCategories::Ewallet, ChannelCategories::RetailOutlet])
            .set_limit(2)
            .build()
        ).await {
            Ok(res) => assert_eq!(res, expected),
            Err(e) => panic!("Error: {}", e)
        }
    }

    #[tokio::test]
    async fn test_get_transaction() {
        dotenv().ok();
        let client = XenditClient::new(std::env::var("XENDIT_API_KEY").unwrap_or("".to_string()));
        let expected = TransactionObject {
            id: String::from("txn_c0f597f6-0361-4cd2-8c79-e195d17cd0e0"),
            product_id: String::from("5c27eea0-4963-41ba-994f-73a810a0b8ea"),
            r#type: String::from("PAYMENT"),
            channel_code: Some(String::from("ID_OVO")),
            reference_id: Some(String::from("order-id-1715068880")),
            account_identifier: Some(String::from("+6285156209625")),
            currency: Some(String::from("IDR")),
            amount: 10000,
            net_amount: 9834,
            cashflow: String::from("MONEY_IN"),
            status: String::from("SUCCESS"),
            channel_category: String::from("EWALLET"),
            business_id: String::from("57fdbb445eec38910d3a4c47"),
            created: "2024-05-07T08:01:21.640Z".parse::<DateTime<Utc>>().unwrap(),
            updated: "2024-05-09T08:01:31.452Z".parse::<DateTime<Utc>>().unwrap(),
            fee: TransactionFee{
                xendit_fee: 150,
                value_added_tax: 16,
                xendit_withholding_tax: 0,
                third_party_withholding_tax: 0,
                status: String::from("COMPLETED")
            },
            settlement_status: Some(String::from("SETTLED")),
            estimated_settlement_time: Some("2024-05-09T08:01:20.281Z".parse::<DateTime<Utc>>().unwrap())
        };
        match TransactionClient::new(&client)
        .get(
            String::from("txn_c0f597f6-0361-4cd2-8c79-e195d17cd0e0")
        ).await {
            Ok(res) => assert_eq!(res, expected),
            Err(e) => panic!("Error: {}", e)
        }
    }
}