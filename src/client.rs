use core::fmt;

use data_encoding::BASE64;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

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
    error_code: Option<String>,
    errors: Option<Vec<ApiErrorDetail>>,
}
impl fmt::Display for ApiErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "message: {}", self.message)?;
        if self.error_code.is_some() {
            writeln!(f, "error_code: {}", self.error_code.as_ref().unwrap())?;
        }
        if self.errors.is_some() {
            writeln!(f, "errors: {:?}", self.errors.as_ref().unwrap())?;
        }
        Ok(())
    }
}

#[skip_serializing_none]
#[derive(Deserialize)]
pub struct ApiErrorDetail {
    field: Option<Vec<String>>,
    location: Option<String>,
    path: Option<String>,
    message: Option<String>,
    messages: Option<Vec<String>>,
    types: Option<Vec<String>>,
}
impl fmt::Debug for ApiErrorDetail {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.field {
            Some(_) => write!(
                f, "\n\tfield: {:?}, location: {:?}, messages: {:?}, types: {:?}",
                self.field.as_ref().unwrap(),
                self.location.as_ref().unwrap(),
                self.messages.as_ref().unwrap(),
                self.types.as_ref().unwrap()
            ),
            None => write!(
                f, "\n\tpath: {:?}, message: {:?}",
                self.path.as_ref().unwrap(),
                self.message.as_ref().unwrap()
            ),
        }
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
            return Err(format!("API Error\n{}", error_response).into());
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
            return Err(format!("API Error\n{}", error_response).into());
        }
        let data = response.json::<T>().await?;
        Ok(data)
    }

    pub(super) async fn post<
        T: for<'de> Deserialize<'de>,
        B: serde::Serialize>
        (&self, endpoint: &str, body: &B) -> Result<T, Box<dyn std::error::Error>> {
        let url: String = format!("{}/{}", API_BASE_URL, endpoint);
        let request_builder: reqwest::RequestBuilder = self.client.post(&url)
            .header("Authorization", format!("Basic {}", self.api_key));
        let response = match serde_json::to_value(body)?.is_null() {
            true => request_builder.send().await?,
            false => request_builder.json(body).send().await?,
        };
        if !vec![reqwest::StatusCode::OK, reqwest::StatusCode::CREATED].contains(&response.status()) {
            let error_response = response.json::<ApiErrorResponse>().await?;
            return Err(format!("API Error\n{}", error_response).into());
        }
        let data = response.json::<T>().await?;
        Ok(data)
    }

    pub(super) async fn post_with_params<
        T: for<'de> Deserialize<'de>,
        B: serde::Serialize,
        P: QueryParams>
        (&self, endpoint: &str, body: &B, params: P) -> Result<T, Box<dyn std::error::Error>> {
        let query_string = params.to_query_params()?;
        let full_url = format!("{}?{}", endpoint, query_string);
        self.post::<T, B>(&full_url, body).await
    }

    pub(super) async fn patch<
        T: for<'de> Deserialize<'de>,
        B: serde::Serialize
        >(&self, endpoint: &str, body: &B) -> Result<T, Box<dyn std::error::Error>> {
        let url: String = format!("{}/{}", API_BASE_URL, endpoint);
        let response = self.client. patch(&url)
            .header("Authorization", format!("Basic {}", self.api_key))
            .json(body)
            .send()
            .await?;
        if response.status() != reqwest::StatusCode::OK {
            let error_response = response.json::<ApiErrorResponse>().await?;
            return Err(format!("API Error\n{}", error_response).into());
        }
        let data = response.json::<T>().await?;
        Ok(data)
    }
}