use serde::{Deserialize, Serialize};

use super::Truncate;

#[derive(Serialize, Debug)]
pub struct EmbedRequest<'input> {
    /// An optional string representing the model you'd like to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<EmbedModel>,
    /// An array of strings for the model to embed.
    pub texts: &'input [String],
    /// Specify how the API will handle inputs longer than the maximum token length.
    pub truncate: Truncate,
}

#[derive(strum_macros::Display, Serialize, Debug)]
pub enum EmbedModel {
    #[strum(serialize = "small")]
    Small,
    #[strum(serialize = "large")]
    Large,
    Custom(String),
}

#[derive(Deserialize, Debug)]
pub(crate) struct EmbedResponse {
    /// An array of embeddings, where each embedding is an array of floats. The length of the embeddings
    /// array will be the same as the length of the original texts array.
    pub embeddings: Vec<Vec<f64>>,
}
