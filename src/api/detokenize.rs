use serde::{Deserialize, Serialize};

use super::GenerateModel;

#[derive(Serialize, Debug)]
pub struct DetokenizeRequest<'input> {
    /// The tokens to be detokenized
    pub tokens: &'input [u64],
    /// optional - The model to use for detokenization. Custom models can also be supplied with their full ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<GenerateModel>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct DetokenizeResponse {
    /// The text representation of the tokens
    pub text: String,
}
