use serde::Serialize;

pub mod chat;
pub mod classify;
pub mod detokenize;
pub mod embed;
pub mod generate;
pub mod rerank;
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
    #[strum(serialize = "embed-english-light-v2.0")]
    #[serde(rename = "embed-english-light-v2.0")]
    EnglishLightV2,
    #[strum(serialize = "embed-english-v2.0")]
    #[serde(rename = "embed-english-v2.0")]
    EnglishV2,
    #[strum(serialize = "embed-multilingual-v2.0")]
    #[serde(rename = "embed-multilingual-v2.0")]
    MultilingualV2,
    #[strum(serialize = "embed-english-light-v3.0")]
    #[serde(rename = "embed-english-light-v3.0")]
    EnglishLightV3,
    #[strum(serialize = "embed-english-v3.0")]
    #[serde(rename = "embed-english-v3.0")]
    EnglishV3,
    #[strum(serialize = "embed-multilingual-v3.0")]
    #[serde(rename = "embed-multilingual-v3.0")]
    MultilingualV3,
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
    #[strum(serialize = "command-r")]
    #[serde(rename = "command-r")]
    CommandR,
    #[strum(serialize = "command-r-plus")]
    #[serde(rename = "command-r-plus")]
    CommandRPlus,
    #[strum(serialize = "command-r-08-2024")]
    #[serde(rename = "command-r-08-2024")]
    CommandR082024,
    #[strum(serialize = "command-r-plus-08-2024")]
    #[serde(rename = "command-r-plus-08-2024")]
    CommandRPlus082024,
    #[strum(serialize = "command-nightly")]
    #[serde(rename = "command-nightly")]
    CommandNightly,
    Custom(String),
}
