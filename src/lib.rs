use std::time::Duration;

use api::{
    chat::{ChatRequest, ChatStreamRequest, ChatStreamResponse},
    classify::{Classification, ClassifyRequest, ClassifyResponse},
    detokenize::{DetokenizeRequest, DetokenizeResponse},
    embed::{EmbedRequest, EmbedResponse},
    generate::{GenerateRequest, GenerateResponse, Generation},
    rerank::{ReRankRequest, ReRankResponse, ReRankResult},
    tokenize::{TokenizeRequest, TokenizeResponse},
};
use reqwest::{header, ClientBuilder, StatusCode, Url};
use tokio::sync::mpsc::{channel, Receiver};

const COHERE_API_BASE_URL: &str = "https://api.cohere.ai";
const COHERE_API_V1: &str = "v1";
const COHERE_API_TIMEOUT: Duration = Duration::from_secs(120);

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

#[derive(Error, Debug)]
pub enum CohereStreamError {
    #[error("Unexpected deserialization error")]
    RequestError(#[from] serde_json::error::Error),
    #[error("Unknown error `{0}`")]
    Unknown(String),
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
        Cohere::new(format!("{COHERE_API_BASE_URL}/{COHERE_API_V1}"), api_key)
    }
}

impl Cohere {
    pub fn new<U: Into<String>, K: Into<String>>(api_url: U, api_key: K) -> Self {
        let api_url: String = api_url.into();
        let api_key: String = api_key.into();

        let mut headers = header::HeaderMap::new();

        let mut authorization = header::HeaderValue::from_str(&format!("Bearer {api_key}"))
            .expect("failed to construct authorization header!");
        authorization.set_sensitive(true);
        headers.insert(header::AUTHORIZATION, authorization);

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
            .timeout(COHERE_API_TIMEOUT)
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

        // Check for any API Warnings
        if let Some(warning) = response.headers().get("X-API-Warning") {
            eprintln!("Warning: {:?}", String::from_utf8_lossy(warning.as_bytes()));
        }

        if response.status().is_client_error() || response.status().is_server_error() {
            let status = response.status();
            let text = response.text().await?;
            Err(CohereApiError::ApiError(
                status,
                serde_json::from_str::<CohereApiErrorResponse>(&text)
                    .unwrap_or(CohereApiErrorResponse {
                        message: format!("Unknown API Error: {}", text),
                    })
                    .message,
            ))
        } else {
            Ok(response.json::<Response>().await?)
        }
    }

    async fn request_stream<Request: Serialize, Response: DeserializeOwned + Send + 'static>(
        &self,
        route: &'static str,
        payload: Request,
    ) -> Result<Receiver<Result<Response, CohereStreamError>>, CohereApiError> {
        let url =
            Url::parse(&format!("{}/{route}", self.api_url)).expect("api url should be valid");

        let mut response = self.client.post(url).json(&payload).send().await?;

        let (tx, rx) = channel::<Result<Response, CohereStreamError>>(32);
        tokio::spawn(async move {
            while let Ok(Some(chunk)) = response.chunk().await {
                if chunk.is_empty() {
                    break;
                }
                match serde_json::from_slice::<Response>(&chunk) {
                    Ok(v) => tx
                        .send(Ok(v))
                        .await
                        .expect("Failed to send message to channel"),
                    Err(e) => tx
                        .send(Err(CohereStreamError::from(e)))
                        .await
                        .expect("Failed to send error to channel"),
                }
            }
        });

        Ok(rx)
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
    pub async fn generate<'input>(
        &self,
        request: &GenerateRequest<'input>,
    ) -> Result<Vec<Generation>, CohereApiError> {
        let response = self
            .request::<_, GenerateResponse>("generate", request)
            .await?;

        Ok(response.generations)
    }

    /// Chat with Cohere's LLM
    pub async fn chat<'input>(
        &self,
        request: &ChatRequest<'input>,
    ) -> Result<Receiver<Result<ChatStreamResponse, CohereStreamError>>, CohereApiError> {
        let stream_request = ChatStreamRequest {
            request,
            stream: true,
        };
        let response = self
            .request_stream::<_, ChatStreamResponse>("chat", stream_request)
            .await?;

        Ok(response)
    }

    /// Returns text embeddings.
    /// An embedding is a list of floating point numbers that captures semantic information about the text that it represents.
    /// Embeddings can be used to create text classifiers as well as empower semantic search.
    pub async fn embed<'input>(
        &self,
        request: &EmbedRequest<'input>,
    ) -> Result<Vec<Vec<f64>>, CohereApiError> {
        let response = self.request::<_, EmbedResponse>("embed", request).await?;

        Ok(response.embeddings)
    }

    /// Makes a prediction about which label fits the specified text inputs best.
    /// To make a prediction, classify uses the provided examples of text + label pairs as a reference.
    pub async fn classify<'input>(
        &self,
        request: &ClassifyRequest<'input>,
    ) -> Result<Vec<Classification>, CohereApiError> {
        let response = self
            .request::<_, ClassifyResponse>("classify", request)
            .await?;

        Ok(response.classifications)
    }

    /// Splits input text into smaller units called tokens using byte-pair encoding (BPE).
    pub async fn tokenize<'input>(
        &self,
        request: &TokenizeRequest<'input>,
    ) -> Result<TokenizeResponse, CohereApiError> {
        let response = self.request("tokenize", request).await?;

        Ok(response)
    }

    /// Takes tokens using byte-pair encoding and returns their text representation.
    pub async fn detokenize<'input>(
        &self,
        request: &DetokenizeRequest<'input>,
    ) -> Result<String, CohereApiError> {
        let response = self
            .request::<_, DetokenizeResponse>("detokenize", request)
            .await?;

        Ok(response.text)
    }

    /// Takes a query plus an list of texts and return an ordered array with each text assigned a relevance score.
    pub async fn rerank<'input>(
        &self,
        request: &ReRankRequest<'input>,
    ) -> Result<Vec<ReRankResult>, CohereApiError> {
        let response = self.request::<_, ReRankResponse>("rerank", request).await?;

        Ok(response.results)
    }
}
