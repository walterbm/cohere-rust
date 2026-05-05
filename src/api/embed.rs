use serde::{Deserialize, Serialize};

use super::{EmbedModel, Truncate};

#[derive(strum_macros::Display, Serialize, Deserialize, Debug, Clone, Copy)]
pub enum EmbedInputType {
    #[strum(serialize = "search_document")]
    #[serde(rename = "search_document")]
    SearchDocument,
    #[strum(serialize = "search_query")]
    #[serde(rename = "search_query")]
    SearchQuery,
    #[strum(serialize = "classification")]
    #[serde(rename = "classification")]
    Classification,
    #[strum(serialize = "clustering")]
    #[serde(rename = "clustering")]
    Clustering,
    #[strum(serialize = "image")]
    #[serde(rename = "image")]
    Image,
}

#[derive(strum_macros::Display, Serialize, Deserialize, Debug, Clone, Copy)]
pub enum EmbeddingType {
    #[strum(serialize = "float")]
    #[serde(rename = "float")]
    Float,
    #[strum(serialize = "int8")]
    #[serde(rename = "int8")]
    Int8,
    #[strum(serialize = "uint8")]
    #[serde(rename = "uint8")]
    Uint8,
    #[strum(serialize = "binary")]
    #[serde(rename = "binary")]
    Binary,
    #[strum(serialize = "ubinary")]
    #[serde(rename = "ubinary")]
    Ubinary,
    #[strum(serialize = "base64")]
    #[serde(rename = "base64")]
    Base64,
}

#[derive(Serialize, Debug)]
pub struct EmbedRequest<'input> {
    /// The model you'd like to use.
    pub model: EmbedModel,
    /// Specifies the type of input passed to the model. Required for embedding models v3 and higher.
    pub input_type: EmbedInputType,
    /// An array of strings for the model to embed.
    pub texts: &'input [String],
    /// Specifies the types of embeddings you want to get back.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub embedding_types: Option<Vec<EmbeddingType>>,
    /// Specify how the API will handle inputs longer than the maximum token length.
    pub truncate: Truncate,
}

impl Default for EmbedRequest<'static> {
    fn default() -> Self {
        Self {
            model: EmbedModel::V4,
            input_type: EmbedInputType::Classification,
            texts: &[],
            embedding_types: None,
            truncate: Truncate::End,
        }
    }
}

#[derive(Deserialize, Debug)]
pub(crate) struct EmbedResponse {
    #[allow(dead_code)]
    pub id: String,
    /// An object with different embedding types.
    pub embeddings: EmbeddingsByType,
    #[allow(dead_code)]
    pub texts: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct EmbeddingsByType {
    pub float: Option<Vec<Vec<f64>>>,
    pub int8: Option<Vec<Vec<i8>>>,
    pub uint8: Option<Vec<Vec<u8>>>,
    pub binary: Option<Vec<Vec<i8>>>,
    pub ubinary: Option<Vec<Vec<u8>>>,
    pub base64: Option<Vec<String>>,
}
