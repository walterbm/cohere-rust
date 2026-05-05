use std::time::Duration;

use api::{
    chat::{ChatRequest, ChatStreamRequest, ChatStreamResponse},
    chat_v2::{ChatV2Request, ChatV2Response, ChatV2StreamEvent},
    classify::{Classification, ClassifyRequest, ClassifyResponse},
    detokenize::{DetokenizeRequest, DetokenizeResponse},
    embed::{EmbedRequest, EmbedResponse},
    generate::{GenerateRequest, GenerateResponse, Generation},
    rerank::{ReRankRequest, ReRankResponse, ReRankResult},
    tokenize::{TokenizeRequest, TokenizeResponse},
};
use reqwest::{ClientBuilder, StatusCode, Url, header};
use tokio::sync::mpsc::{Receiver, channel};

const COHERE_API_BASE_URL: &str = "https://api.cohere.com";
const COHERE_API_V1: &str = "v1";
const COHERE_API_TIMEOUT: Duration = Duration::from_secs(240);

use serde::{Deserialize, Serialize, de::DeserializeOwned};
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
    #[error("Deserialization error: {0}")]
    DeserializationError(String),
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
///
/// Supports both v1 and v2 API endpoints. Use `chat()` for v1 streaming chat, or
/// `chat_v2()` / `chat_v2_non_stream()` for the modern v2 messages-based API.
#[derive(Clone)]
pub struct Cohere {
    api_url: String,
    api_url_v2: String,
    client: reqwest::Client,
}

impl std::fmt::Debug for Cohere {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cohere")
            .field("api_url", &self.api_url)
            .field("api_url_v2", &self.api_url_v2)
            .finish_non_exhaustive()
    }
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

        let api_url_v2 = if api_url.ends_with("/v1") {
            format!("{}v2", &api_url[..api_url.len() - 2])
        } else if api_url.contains("/v1/") {
            api_url.replacen("/v1/", "/v2/", 1)
        } else {
            format!("{}/v2", api_url.trim_end_matches('/'))
        };

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

        Cohere {
            api_url,
            api_url_v2,
            client,
        }
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
            Err(self.parse_error(response).await)
        } else {
            Ok(response.json::<Response>().await?)
        }
    }

    async fn request_v2<Request: Serialize, Response: DeserializeOwned>(
        &self,
        route: &str,
        payload: Request,
    ) -> Result<Response, CohereApiError> {
        let url =
            Url::parse(&format!("{}/{route}", self.api_url_v2)).expect("api url should be valid");

        let response = self.client.post(url).json(&payload).send().await?;

        if let Some(warning) = response.headers().get("X-API-Warning") {
            eprintln!("Warning: {:?}", String::from_utf8_lossy(warning.as_bytes()));
        }

        if response.status().is_client_error() || response.status().is_server_error() {
            Err(self.parse_error(response).await)
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

        if response.status().is_client_error() || response.status().is_server_error() {
            return Err(self.parse_error(response).await);
        }

        let (tx, rx) = channel::<Result<Response, CohereStreamError>>(1);
        tokio::spawn(async move {
            let mut buf = bytes::BytesMut::with_capacity(1024);
            while let Ok(Some(chunk)) = response.chunk().await {
                if chunk.is_empty() {
                    break;
                }
                buf.extend_from_slice(&chunk);
                if !chunk.ends_with(b"\n") {
                    continue;
                }
                match serde_json::from_slice::<Response>(&buf) {
                    Ok(v) => tx
                        .send(Ok(v))
                        .await
                        .expect("Failed to send message to channel"),
                    Err(e) => tx
                        .send(Err(CohereStreamError::from(e)))
                        .await
                        .expect("Failed to send error to channel"),
                }
                buf.clear()
            }
        });

        Ok(rx)
    }

    /// Stream a v2 request using SSE (Server-Sent Events) format.
    async fn request_stream_v2<Request: Serialize, Response: DeserializeOwned + Send + 'static>(
        &self,
        route: &str,
        payload: Request,
    ) -> Result<Receiver<Result<Response, CohereStreamError>>, CohereApiError> {
        let url =
            Url::parse(&format!("{}/{route}", self.api_url_v2)).expect("api url should be valid");

        let response = self.client.post(url).json(&payload).send().await?;

        if response.status().is_client_error() || response.status().is_server_error() {
            return Err(self.parse_error(response).await);
        }

        let (tx, rx) = channel::<Result<Response, CohereStreamError>>(1);
        tokio::spawn(async move {
            let mut buf = bytes::BytesMut::with_capacity(4096);
            let mut response = response;
            while let Ok(Some(chunk)) = response.chunk().await {
                if chunk.is_empty() {
                    break;
                }
                buf.extend_from_slice(&chunk);

                while let Some(newline_pos) = buf.iter().position(|&b| b == b'\n') {
                    let line_bytes = buf.split_to(newline_pos + 1);
                    let line = String::from_utf8_lossy(&line_bytes).trim().to_string();

                    if line.is_empty() {
                        continue;
                    }

                    let json_str = if let Some(stripped) = line.strip_prefix("data: ") {
                        stripped
                    } else {
                        &line
                    };

                    if json_str.is_empty() {
                        continue;
                    }

                    match serde_json::from_str::<Response>(json_str) {
                        Ok(v) => {
                            if tx.send(Ok(v)).await.is_err() {
                                return;
                            }
                        }
                        Err(e) => {
                            if tx.send(Err(CohereStreamError::from(e))).await.is_err() {
                                return;
                            }
                        }
                    }
                }
            }
        });

        Ok(rx)
    }

    async fn parse_error(&self, response: reqwest::Response) -> CohereApiError {
        let status = response.status();
        let text = response.text().await;
        match text {
            Err(_) => CohereApiError::Unknown,
            Ok(text) => CohereApiError::ApiError(
                status,
                serde_json::from_str::<CohereApiErrorResponse>(&text)
                    .unwrap_or(CohereApiErrorResponse {
                        message: format!("Unknown API Error: {}", text),
                    })
                    .message,
            ),
        }
    }

    /// Verify that the Cohere API key being used is valid. Uses the v2 check-api-key endpoint.
    pub async fn check_api_key(&self) -> Result<(), CohereApiError> {
        let response = self
            .request_v2::<(), CohereCheckApiKeyResponse>("check-api-key", ())
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

    /// Chat with Cohere's LLM using the v1 streaming API.
    ///
    /// For the modern messages-based API, use `chat_v2()` or `chat_v2_non_stream()`.
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
        let response = self
            .request_v2::<_, EmbedResponse>("embed", request)
            .await?;

        match response.embeddings.float {
            Some(f) => Ok(f),
            None => Err(CohereApiError::DeserializationError(
                "No float embeddings found in response".to_string(),
            )),
        }
    }

    /// Makes a prediction about which label fits the specified text inputs best.
    /// To make a prediction, classify uses the provided examples of text + label pairs as a reference.
    pub async fn classify<'input>(
        &self,
        request: &ClassifyRequest<'input>,
    ) -> Result<Vec<Classification>, CohereApiError> {
        let response = self
            .request_v2::<_, ClassifyResponse>("classify", request)
            .await?;

        Ok(response.classifications)
    }

    /// Splits input text into smaller units called tokens using byte-pair encoding (BPE).
    pub async fn tokenize<'input>(
        &self,
        request: &TokenizeRequest<'input>,
    ) -> Result<TokenizeResponse, CohereApiError> {
        let response = self.request_v2("tokenize", request).await?;

        Ok(response)
    }

    /// Takes tokens using byte-pair encoding and returns their text representation.
    pub async fn detokenize<'input>(
        &self,
        request: &DetokenizeRequest<'input>,
    ) -> Result<String, CohereApiError> {
        let response = self
            .request_v2::<_, DetokenizeResponse>("detokenize", request)
            .await?;

        Ok(response.text)
    }

    /// Takes a query plus an list of texts and return an ordered array with each text assigned a relevance score.
    pub async fn rerank<'input>(
        &self,
        request: &ReRankRequest<'input>,
    ) -> Result<Vec<ReRankResult>, CohereApiError> {
        let response = self
            .request_v2::<_, ReRankResponse>("rerank", request)
            .await?;

        Ok(response.results)
    }

    /// Chat using the v2 messages-based API (non-streaming).
    ///
    /// Returns a complete `ChatV2Response` with the assistant's message, citations, and usage info.
    pub async fn chat_v2_non_stream(
        &self,
        request: &ChatV2Request,
    ) -> Result<ChatV2Response, CohereApiError> {
        let mut payload = request.clone();
        payload.stream = Some(false);
        self.request_v2::<_, ChatV2Response>("chat", payload).await
    }

    /// Chat using the v2 messages-based API (streaming).
    ///
    /// Returns a channel receiver that yields `ChatV2StreamEvent` for each SSE event.
    /// Use `event.text()` to extract text content from `content-delta` events.
    pub async fn chat_v2(
        &self,
        request: &ChatV2Request,
    ) -> Result<Receiver<Result<ChatV2StreamEvent, CohereStreamError>>, CohereApiError> {
        let mut payload = request.clone();
        payload.stream = Some(true);
        self.request_stream_v2::<_, ChatV2StreamEvent>("chat", payload)
            .await
    }
}
