use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct TokenizeRequest<'input> {
    /// The string to be tokenized
    pub text: &'input str,
}

#[derive(Deserialize, Debug)]
pub struct TokenizeResponse {
    /// The tokens
    pub tokens: Vec<u64>,
    /// String representations of the tokens
    pub token_strings: Vec<String>,
}
