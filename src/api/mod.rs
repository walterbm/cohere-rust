use serde::Serialize;

pub mod classify;
pub mod detect_language;
pub mod detokenize;
pub mod embed;
pub mod generate;
pub mod rerank;
pub mod summarize;
pub mod tokenize;

#[derive(strum_macros::Display, Serialize, Debug)]
pub enum Truncate {
    #[strum(serialize = "NONE")]
    #[serde(rename = "NONE")]
    None,
    #[strum(serialize = "START")]
    #[serde(rename = "START")]
    Start,
    #[strum(serialize = "END")]
    #[serde(rename = "END")]
    End,
}

#[derive(strum_macros::Display, Serialize, Debug)]
pub enum EmbedModel {
    // Lighter and faster english model with an embedding vector of size 1024
    #[strum(serialize = "embed-english-light-v2.0")]
    #[serde(rename = "embed-english-light-v2.0")]
    EmbedEnglishLight,
    // Default model with an embedding vector of size 4096
    #[strum(serialize = "embed-english-v2.0")]
    #[serde(rename = "embed-english-v2.0")]
    EmbedEnglish,
    // Multi-language model with an embedding vector of size 768
    #[strum(serialize = "embed-multilingual-v2.0")]
    #[serde(rename = "embed-multilingual-v2.0")]
    EmbedMultilingual,
    // Custom model
    Custom(String),
}
