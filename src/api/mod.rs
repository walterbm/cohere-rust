use serde::Serialize;

pub mod chat;
pub mod chat_v2;
pub mod classify;
pub mod detokenize;
pub mod embed;
pub mod generate;
pub mod rerank;
pub mod tokenize;
pub use embed::{EmbedInputType, EmbeddingType};

/// Modern Cohere chat models for the Chat API.
#[derive(strum_macros::Display, Serialize, Debug, Clone)]
pub enum ChatModel {
    #[strum(serialize = "command-a-03-2025")]
    #[serde(rename = "command-a-03-2025")]
    CommandA032025,
    #[strum(serialize = "command-r7b-12-2024")]
    #[serde(rename = "command-r7b-12-2024")]
    CommandR7b122024,
    #[strum(serialize = "command-a-translate-08-2025")]
    #[serde(rename = "command-a-translate-08-2025")]
    CommandATranslate082025,
    #[strum(serialize = "command-a-reasoning-08-2025")]
    #[serde(rename = "command-a-reasoning-08-2025")]
    CommandAReasoning082025,
    #[strum(serialize = "command-a-vision-07-2025")]
    #[serde(rename = "command-a-vision-07-2025")]
    CommandAVision072025,
    #[strum(serialize = "command-r-08-2024")]
    #[serde(rename = "command-r-08-2024")]
    CommandR082024,
    #[strum(serialize = "command-r-plus-08-2024")]
    #[serde(rename = "command-r-plus-08-2024")]
    CommandRPlus082024,
    #[strum(serialize = "c4ai-aya-expanse-32b")]
    #[serde(rename = "c4ai-aya-expanse-32b")]
    AyaExpanse32b,
    #[strum(serialize = "c4ai-aya-vision-32b")]
    #[serde(rename = "c4ai-aya-vision-32b")]
    AyaVision32b,
    #[strum(serialize = "tiny-aya-global")]
    #[serde(rename = "tiny-aya-global")]
    TinyAyaGlobal,
    #[strum(serialize = "tiny-aya-earth")]
    #[serde(rename = "tiny-aya-earth")]
    TinyAyaEarth,
    #[strum(serialize = "tiny-aya-fire")]
    #[serde(rename = "tiny-aya-fire")]
    TinyAyaFire,
    #[strum(serialize = "tiny-aya-water")]
    #[serde(rename = "tiny-aya-water")]
    TinyAyaWater,
    Custom(String),
}

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
    #[strum(serialize = "embed-v4.0")]
    #[serde(rename = "embed-v4.0")]
    V4,
    #[strum(serialize = "embed-english-v3.0")]
    #[serde(rename = "embed-english-v3.0")]
    EnglishV3,
    #[strum(serialize = "embed-english-light-v3.0")]
    #[serde(rename = "embed-english-light-v3.0")]
    EnglishLightV3,
    #[strum(serialize = "embed-multilingual-v3.0")]
    #[serde(rename = "embed-multilingual-v3.0")]
    MultilingualV3,
    #[strum(serialize = "embed-multilingual-light-v3.0")]
    #[serde(rename = "embed-multilingual-light-v3.0")]
    MultilingualLightV3,
    /// Custom model name.
    Custom(String),
}

/// Models for the v1 Generate and Chat endpoints.
///
/// Note: The v1 generate endpoint is deprecated. Prefer the v2 Chat API
/// with `ChatModel` for new code.
#[derive(strum_macros::Display, Serialize, Debug)]
pub enum GenerateModel {
    #[strum(serialize = "command-a-03-2025")]
    #[serde(rename = "command-a-03-2025")]
    CommandA032025,
    #[strum(serialize = "command-r7b-12-2024")]
    #[serde(rename = "command-r7b-12-2024")]
    CommandR7b122024,
    #[strum(serialize = "command-r-08-2024")]
    #[serde(rename = "command-r-08-2024")]
    CommandR082024,
    #[strum(serialize = "command-r-plus-08-2024")]
    #[serde(rename = "command-r-plus-08-2024")]
    CommandRPlus082024,
    /// Custom model name.
    Custom(String),
}
