use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct TokenizeRequest<'input> {
    /// The string to be tokenized
    pub text: &'input str,
}

#[derive(Deserialize, Debug)]
pub struct TokenizeResponse {
    /// An array of integers, where each integer represents a single token.
    pub tokens: Vec<u32>,
}
