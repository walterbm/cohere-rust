use serde::{Deserialize, Serialize};

use super::GenerateModel;

#[derive(Serialize, Default, Debug)]
pub struct ChatRequest<'input> {
    /// The chat message from the user to the model.
    pub message: &'input str,
    /// optional - The model to use for text generation. Custom models can also be supplied with their full ID. Defaults to 'command'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<GenerateModel>,
    /// optional - Dictates how the prompt will be constructed. When set to 'AUTO' some parts of chat history and documents will be dropped
    /// to construct a prompt that fits within the model's context length limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_truncation: Option<PromptTruncation>,
    /// optional - A non-negative float that tunes the degree of randomness in generation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    /// optional - Dictates the approach taken to generating citations during RAG chat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub citation_quality: Option<CitationQuality>,
    /// optional - Previous conversations can be stored and resumed by providing the conversation's identifier.
    /// If a conversation with this id does not already exist, a new conversation will be created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,
}

#[derive(strum_macros::Display, Serialize, Debug)]
pub enum PromptTruncation {
    #[strum(serialize = "AUTO")]
    #[serde(rename = "AUTO")]
    Auto,
    #[strum(serialize = "OFF")]
    #[serde(rename = "OFF")]
    Off,
}

#[derive(strum_macros::Display, Serialize, Debug)]
pub enum CitationQuality {
    #[strum(serialize = "accurate")]
    #[serde(rename = "accurate")]
    Accurate,
    #[strum(serialize = "fast")]
    #[serde(rename = "fast")]
    Fast,
}

#[derive(Deserialize, Debug)]
pub(crate) struct ChatResponse {
    /// Chat response text
    pub text: String,
}
