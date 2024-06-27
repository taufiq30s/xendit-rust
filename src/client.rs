use core::fmt;

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

// Create Struct to Handle Error Responses
// from API
#[derive(Deserialize)]
pub struct ApiErrorResponse {
    message: String,
    error_code: String,
    errors: Vec<ApiErrorDetail>,
}
impl fmt::Display for ApiErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\nmessage: {}, error_code: {}, errors: \n{:?}", self.message, self.error_code, self.errors)
    }
}

#[derive(Deserialize)]
pub struct ApiErrorDetail {
    field: Vec<String>,
    location: String,
    messages: Vec<String>,
    types: Vec<String>,
}
impl fmt::Debug for ApiErrorDetail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n\tfield: {:?}, location: {}, messages: {:?}, types: {:?}", self.field, self.location, self.messages, self.types)
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
        if response.status() != reqwest::StatusCode::OK {
            let error_response = response.json::<ApiErrorResponse>().await?;
            return Err(format!("API Error: {}", error_response).into());
        }
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
        if response.status() != reqwest::StatusCode::OK {
            let error_response = response.json::<ApiErrorResponse>().await?;
            return Err(format!("API Error: {}", error_response).into());
        }
        let data = response.json::<T>().await?;
        Ok(data)
    }

    pub(super) async fn post<
        T: for<'de> Deserialize<'de>,
        B: serde::Serialize
        >(&self, endpoint: &str, body: &B) -> Result<T, Box<dyn std::error::Error>> {
        let url: String = format!("{}{}", API_BASE_URL, endpoint);
        let request_builder: reqwest::RequestBuilder = self.client.post(&url)
            .header("Authorization", format!("Basic {}", self.api_key));
        let response = match serde_json::to_value(body)?.is_null() {
            true => request_builder.send().await?,
            false => request_builder.json(body).send().await?,
        };
        if response.status() != reqwest::StatusCode::OK {
            let error_response = response.json::<ApiErrorResponse>().await?;
            return Err(format!("API Error: {}", error_response).into());
        }
        let data = response.json::<T>().await?;
        Ok(data)
    }
}