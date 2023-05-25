use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::Truncate;

#[derive(Serialize, Default, Debug)]
pub struct GenerateRequest<'input> {
    /// Represents the prompt or text to be completed.
    pub prompt: &'input str,
    /// optional - The model to use for text generation. Custom models can also be supplied with their full ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<GenerateModel>,
    /// optional - Denotes the number of tokens to predict per generation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    /// optional - The ID of a custom playground preset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preset: Option<String>,
    /// optional - A non-negative float that tunes the degree of randomness in generation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    /// optional - Denotes the maximum number of generations that will be returned. Defaults to 1,
    /// max value of 5.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub num_generations: Option<u8>,
    /// optional - If set to a positive integer, it ensures only the top k most likely tokens are
    /// considered for generation at each step. Defaults to 0 (disabled)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub k: Option<u64>,
    /// optional - If set to a probability 0.0 < p < 1.0, it ensures that only the most likely tokens,
    /// with total probability mass of p, are considered for generation at each step. If both k and
    /// p are enabled, p acts after k. Max value of 1.0. Defaults to 0.75.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p: Option<f64>,
    /// optional - Can be used to reduce repetitiveness of generated tokens. The higher the value,
    /// the stronger a penalty is applied to previously present tokens, proportional to how many
    /// times they have already appeared in the prompt or prior generation. Max value of 1.0. Defaults to 0.0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f64>,
    /// optional - Can be used to reduce repetitiveness of generated tokens. Similar to frequency_penalty,
    /// except that this penalty is applied equally to all tokens that have already appeared, regardless
    /// of their exact frequencies. Max value of 1.0. Defaults to 0.0.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f64>,
    /// optional - The generated text will be cut at the beginning of the earliest occurrence of an end sequence.
    /// The sequence will be excluded from the text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_sequences: Option<Vec<String>>,
    /// optional - The generated text will be cut at the end of the earliest occurrence of a stop sequence.
    /// The sequence will be included the text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_sequences: Option<Vec<String>>,
    /// optional - One of GENERATION|ALL|NONE to specify how and if the token likelihoods are returned with
    /// the response. If GENERATION is selected, the token likelihoods will only be provided for generated
    /// text. If ALL is selected, the token likelihoods will be provided both for the prompt and the generated
    /// text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_likelihoods: Option<ReturnLikelihoods>,
    /// optional - Used to prevent the model from generating unwanted tokens or to incentivize it to include desired tokens
    /// A map of tokens to biases where bias is a float between -10 and +10
    /// Negative values will disincentivize that token from appearing while positives values will incentivize them
    /// Tokens can be obtained from text using the tokenizer
    /// Note: logit bias may not be supported for all finetune models
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<HashMap<u64, f32>>,
    /// optional - Specify how the API will handle inputs longer than the maximum token length.
    /// Passing START will discard the start of the input. END will discard the end of the input.
    /// In both cases, input is discarded until the remaining input is exactly the maximum input token length for the model.
    /// If NONE is selected, when the input exceeds the maximum input token length an error will be returned.
    pub truncate: Option<Truncate>,
}

#[derive(strum_macros::Display, Serialize, Debug)]
pub enum GenerateModel {
    #[strum(serialize = "command")]
    #[serde(rename = "command")]
    Command,
    #[strum(serialize = "command-light")]
    #[serde(rename = "command-light")]
    CommandLight,
    #[strum(serialize = "command-nightly")]
    #[serde(rename = "command-nightly")]
    CommandNightly,
    #[strum(serialize = "command-light-nightly")]
    #[serde(rename = "command-light-nightly")]
    CommandLightNightly,
    Custom(String),
}

#[derive(strum_macros::Display, Serialize, Debug)]
pub enum ReturnLikelihoods {
    #[strum(serialize = "GENERATION")]
    #[serde(rename = "GENERATION")]
    Generation,
    #[strum(serialize = "ALL")]
    #[serde(rename = "ALL")]
    All,
    #[strum(serialize = "NONE")]
    #[serde(rename = "NONE")]
    None,
}

#[derive(Deserialize, Debug)]
pub(crate) struct GenerateResponse {
    /// Contains the generations.
    pub generations: Vec<Generation>,
}

#[derive(Deserialize, Debug)]
pub struct Generation {
    /// Contains the generated text.
    pub text: String,
    /// The sum of the log-likelihood of each token in the string.
    #[serde(default)]
    pub likelihood: f64,
    /// Only returned if `return_likelihoods` is not set to NONE.
    /// The likelihood.
    #[serde(default)]
    pub token_likelihoods: Vec<TokenLikelihood>,
}

#[derive(Deserialize, Debug)]
pub struct TokenLikelihood {
    /// The token.
    pub token: String,
    /// Refers to the log-likelihood of the token. The first token of a context will not
    /// have a likelihood.
    pub likelihood: String,
}
