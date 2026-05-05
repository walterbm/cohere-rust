use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Default)]
pub struct ReRankRequest<'input> {
    /// The search query.
    pub query: &'input str,
    /// A list of document strings to rerank.
    pub documents: &'input [String],
    /// The model to use.
    pub model: ReRankModel,
    /// The number of results to return, defaults to the length of the documents.
    pub top_n: Option<u64>,
    // The maximum number of chunks to derive from each document.
    pub max_chunks_per_doc: Option<u64>,
}

#[derive(strum_macros::Display, Serialize, Debug, Default)]
pub enum ReRankModel {
    #[strum(serialize = "rerank-v4.0-pro")]
    #[serde(rename = "rerank-v4.0-pro")]
    #[default]
    RerankV4Pro,
    #[strum(serialize = "rerank-v4.0-fast")]
    #[serde(rename = "rerank-v4.0-fast")]
    RerankV4Fast,
    #[strum(serialize = "rerank-v3.5")]
    #[serde(rename = "rerank-v3.5")]
    RerankV35,
    #[strum(serialize = "rerank-english-v3.0")]
    #[serde(rename = "rerank-english-v3.0")]
    EnglishV3,
    #[strum(serialize = "rerank-multilingual-v3.0")]
    #[serde(rename = "rerank-multilingual-v3.0")]
    MultilingualV3,
    /// Custom model name.
    Custom(String),
}

#[derive(Deserialize, Debug)]
pub(crate) struct ReRankResponse {
    /// List of ranked documents
    pub results: Vec<ReRankResult>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct ReRankResult {
    /// The index of the input document
    pub index: u64,
    /// A relevance score assigned to the ranking
    pub relevance_score: f64,
}
