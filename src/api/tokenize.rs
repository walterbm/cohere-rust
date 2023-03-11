use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct TokenizeRequest {
    /// The string to be tokenized
    pub text: String,
}

#[derive(Deserialize, Debug)]
pub struct TokenizeResponse {
    /// The tokens
    pub tokens: Vec<u64>,
    /// String representations of the tokens
    pub token_strings: Vec<String>,
}
