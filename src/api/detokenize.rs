use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct DetokenizeRequest {
    /// The tokens to be detokenized
    pub tokens: Vec<u64>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct DetokenizeResponse {
    /// The text representation of the tokens
    pub text: String,
}
