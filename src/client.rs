use data_encoding::BASE64;
use reqwest::Client;
use serde::{Deserialize, Serialize};

const API_BASE_URL: &str = "https://api.xendit.co";

// Create trait to process struct of parameters
// to URL Encoded format
pub trait QueryParams {
    fn to_query_params(&self) -> Result<String, String>;
}
impl<T:Serialize> QueryParams for T {
    fn to_query_params(&self) -> Result<String, String> {
        serde_qs::to_string(self).map_err(
            |e| 
            format!("Failed to serialize query parameters: {}", e))
    }
}

// Create struct of client
pub struct XenditClient {
    client: Client,
    api_key: String
}

impl XenditClient {
    pub fn new(api_key: String) -> Self {
        XenditClient {
            client: Client::new(),
            api_key: BASE64.encode(format!("{}:", api_key).as_bytes())
        }
    }

    // Execute API using Reqwest
    pub(super) async fn get<T: for<'de> Deserialize<'de>>(&self, endpoint: &str) -> Result<T, Box<dyn std::error::Error>> {
        let url: String = format!("{}/{}", API_BASE_URL, endpoint);
        let response = self.client.get(&url)
            .header("Authorization", format!("Basic {}", self.api_key))
            .send()
            .await?;
        let data = response.json::<T>().await?;
        Ok(data)
    }

    pub(super) async fn get_with_params<
        T: for<'de> Deserialize<'de>,
        P: QueryParams>
        (&self, endpoint: &str, params: P) -> Result<T, Box<dyn std::error::Error>> {
        let url = format!("{}/{}", API_BASE_URL, endpoint);
        let query_string = params.to_query_params()?;
        let full_url = format!("{}?{}", url, query_string);
        let response = self.client.get(&full_url)
            .header("Authorization", format!("Basic {}", self.api_key))
            .send()
            .await?;
        let data = response.json::<T>().await?;
        Ok(data)
    }

    pub(super) async fn post<
        T: for<'de> Deserialize<'de>,
        B: serde::Serialize
        >(&self, endpoint: &str, body: &B) -> Result<T, Box<dyn std::error::Error>> {
        let url: String = format!("{}{}", API_BASE_URL, endpoint);
        let response = self.client.post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(body)
            .send()
            .await?;
        let data = response.json::<T>().await?;
        Ok(data)
    }
}