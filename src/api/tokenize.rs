use serde::{Deserialize, Serialize};

use super::GenerateModel;

#[derive(Serialize, Debug)]
pub struct TokenizeRequest<'input> {
    /// The string to be tokenized
    pub text: &'input str,
    /// optional - The model to use for tokenization. Custom models can also be supplied with their full ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<GenerateModel>,
}

#[derive(Deserialize, Debug)]
pub struct TokenizeResponse {
    /// The tokens
    pub tokens: Vec<u64>,
    /// String representations of the tokens
    pub token_strings: Vec<String>,
}
