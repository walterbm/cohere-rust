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
    EnglishLight,
    // Default model with an embedding vector of size 4096
    #[strum(serialize = "embed-english-v2.0")]
    #[serde(rename = "embed-english-v2.0")]
    English,
    // Multi-language model with an embedding vector of size 768
    #[strum(serialize = "embed-multilingual-v2.0")]
    #[serde(rename = "embed-multilingual-v2.0")]
    Multilingual,
    // Custom model
    Custom(String),
}

#[derive(strum_macros::Display, Serialize, Debug)]
pub enum GenerateModel {
    #[strum(serialize = "command")]
    #[serde(rename = "command")]
    Command,
    #[strum(serialize = "command-light")]
    #[serde(rename = "command-light")]
    CommandLight,
    #[strum(serialize = "command-nightly")]
    #[serde(rename = "command-nightly")]
    CommandNightly,
    #[strum(serialize = "command-light-nightly")]
    #[serde(rename = "command-light-nightly")]
    CommandLightNightly,
    Custom(String),
}
