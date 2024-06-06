use reqwest::Client;
use serde::Deserialize;

const API_BASE_URL: &str = "https://api.xendit.co/";
pub struct APIClient {
    client: Client,
    api_key: String
}

impl APIClient {
    pub fn new(api_key: String) -> Self {
        APIClient {
            client: Client::new(),
            api_key
        }
    }

    // Execute API using Reqwest
    pub async fn get<T: for<'de> Deserialize<'de>>(&self, endpoint: &str) -> Result<T, reqwest::Error> {
        let url: String = format!("{}{}", API_BASE_URL, endpoint);
        let response = self.client.get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await;
        match response {
            Ok(response) => {
                let data = response.json::<T>().await?;
                Ok(data)
            },
            Err(error) => {
                Err(error)
            }
        }
    }

    pub async fn post<
        T: for<'de> Deserialize<'de>,
        B: serde::Serialize
        >(&self, endpoint: &str, body: &B) -> Result<T, reqwest::Error> {
        let url: String = format!("{}{}", API_BASE_URL, endpoint);
        let response = self.client.post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(body)
            .send()
            .await;
        match response {
            Ok(response) => {
                let data = response.json::<T>().await?;
                Ok(data)
            },
            Err(error) => {
                Err(error)
            }
        }
    }
}