use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct DetokenizeRequest<'input> {
    /// The tokens to be detokenized
    pub tokens: &'input [u32],
}

#[derive(Deserialize, Debug)]
pub struct DetokenizeResponse {
    /// The detokenized string
    pub text: String,
}
