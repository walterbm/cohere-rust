use std::time::Duration;

use api::{
    classify::{Classification, ClassifyRequest, ClassifyResponse},
    detect_language::{DetectLanguageRequest, DetectLanguageResponse, DetectLanguageResult},
    detokenize::{DetokenizeRequest, DetokenizeResponse},
    embed::{EmbedRequest, EmbedResponse},
    generate::{GenerateRequest, GenerateResponse, Generation},
    summarize::{SummarizeRequest, SummarizeResponse},
    tokenize::{TokenizeRequest, TokenizeResponse},
};
use reqwest::{header, ClientBuilder, StatusCode, Url};

const COHERE_API_BASE_URL: &str = "https://api.cohere.ai";
const COHERE_API_LATEST_VERSION: &str = "2022-12-06";

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use thiserror::Error;

pub mod api;

#[derive(Error, Debug)]
pub enum CohereApiError {
    #[error("Unexpected request error")]
    RequestError(#[from] reqwest::Error),
    #[error("API request failed with status code `{0}` and error message `{1}`")]
    ApiError(StatusCode, String),
    #[error("API key is invalid")]
    InvalidApiKey,
    #[error("Unknown error")]
    Unknown,
}

/// Cohere Rust SDK to build natural language understanding and generation into your product with a few lines of code.
pub struct Cohere {
    api_url: String,
    client: reqwest::Client,
}

#[derive(Deserialize, Debug)]
struct CohereCheckApiKeyResponse {
    valid: bool,
}

#[derive(Deserialize, Debug)]
struct CohereApiErrorResponse {
    message: String,
}

impl Default for Cohere {
    fn default() -> Self {
        let api_key = std::env::var("COHERE_API_KEY")
            .expect("please provide a Cohere API key with the 'COHERE_API_KEY' env variable");
        Cohere::new(COHERE_API_BASE_URL, api_key, COHERE_API_LATEST_VERSION)
    }
}

impl Cohere {
    pub fn new<U: Into<String>, K: Into<String>, V: Into<String>>(
        api_url: U,
        api_key: K,
        version: V,
    ) -> Self {
        let api_url: String = api_url.into();
        let api_key: String = api_key.into();
        let version: String = version.into();

        let mut headers = header::HeaderMap::new();

        let mut authorization = header::HeaderValue::from_str(&format!("Bearer {api_key}"))
            .expect("failed to construct authorization header!");
        authorization.set_sensitive(true);
        headers.insert(header::AUTHORIZATION, authorization);

        headers.insert(
            "Cohere-Version",
            header::HeaderValue::from_str(&version)
                .expect("failed to construct cohere version header!"),
        );

        headers.insert(
            "Request-Source",
            header::HeaderValue::from_static("rust-sdk"),
        );

        headers.insert(
            header::ACCEPT,
            header::HeaderValue::from_static("application/json"),
        );
        headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );

        let client = ClientBuilder::new()
            .default_headers(headers)
            .use_rustls_tls()
            .timeout(Duration::from_secs(5))
            .build()
            .expect("failed to initialize HTTP client!");

        Cohere { api_url, client }
    }

    async fn request<Request: Serialize, Response: DeserializeOwned>(
        &self,
        route: &'static str,
        payload: Request,
    ) -> Result<Response, CohereApiError> {
        let url =
            Url::parse(&format!("{}/{route}", self.api_url)).expect("api url should be valid");

        let response = self.client.post(url).json(&payload).send().await?;

        if response.status().is_client_error() || response.status().is_server_error() {
            Err(CohereApiError::ApiError(
                response.status(),
                response
                    .json::<CohereApiErrorResponse>()
                    .await
                    .unwrap_or(CohereApiErrorResponse {
                        message: "Unknown API Error".to_string(),
                    })
                    .message,
            ))
        } else {
            Ok(response.json::<Response>().await?)
        }
    }

    /// Verify that the Cohere API key being used is valid
    pub async fn check_api_key(&self) -> Result<(), CohereApiError> {
        let response = self
            .request::<(), CohereCheckApiKeyResponse>("check-api-key", ())
            .await?;

        match response.valid {
            true => Ok(()),
            false => Err(CohereApiError::InvalidApiKey),
        }
    }

    /// Generates realistic text conditioned on a given input.
    pub async fn generate(
        &self,
        request: &GenerateRequest,
    ) -> Result<Vec<Generation>, CohereApiError> {
        let response = self
            .request::<_, GenerateResponse>("generate", request)
            .await?;

        Ok(response.generations)
    }

    /// Returns text embeddings.
    /// An embedding is a list of floating point numbers that captures semantic information about the text that it represents.
    /// Embeddings can be used to create text classifiers as well as empower semantic search.
    pub async fn embed(&self, request: &EmbedRequest) -> Result<Vec<Vec<f64>>, CohereApiError> {
        let response = self.request::<_, EmbedResponse>("embed", request).await?;

        Ok(response.embeddings)
    }

    /// Makes a prediction about which label fits the specified text inputs best.
    /// To make a prediction, classify uses the provided examples of text + label pairs as a reference.
    pub async fn classify(
        &self,
        request: &ClassifyRequest,
    ) -> Result<Vec<Classification>, CohereApiError> {
        let response = self
            .request::<_, ClassifyResponse>("classify", request)
            .await?;

        Ok(response.classifications)
    }

    /// Generates a summary in English for a given text.
    pub async fn summarize(&self, request: &SummarizeRequest) -> Result<String, CohereApiError> {
        let response = self
            .request::<_, SummarizeResponse>("summarize", request)
            .await?;

        Ok(response.summary)
    }

    /// Splits input text into smaller units called tokens using byte-pair encoding (BPE).
    pub async fn tokenize(
        &self,
        request: &TokenizeRequest,
    ) -> Result<TokenizeResponse, CohereApiError> {
        let response = self.request("tokenize", request).await?;

        Ok(response)
    }

    /// Takes tokens using byte-pair encoding and returns their text representation.
    pub async fn detokenize(&self, request: &DetokenizeRequest) -> Result<String, CohereApiError> {
        let response = self
            .request::<_, DetokenizeResponse>("detokenize", request)
            .await?;

        Ok(response.text)
    }

    /// Identifies which language each of the provided texts is written in
    pub async fn detect_language(
        &self,
        request: &DetectLanguageRequest,
    ) -> Result<Vec<DetectLanguageResult>, CohereApiError> {
        let response = self
            .request::<_, DetectLanguageResponse>("detect-language", request)
            .await?;

        Ok(response.results)
    }
}
