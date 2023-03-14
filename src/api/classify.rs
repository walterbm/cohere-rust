use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Default, Debug)]
pub struct ClassifyRequest<'input> {
    /// An optional string representing the model you'd like to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<ClassifyModel>,
    /// An optional string representing the ID of a custom playground preset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preset: Option<String>,
    /// An array of strings that you would like to classify.
    pub inputs: &'input [String],
    /// An array of ClassifyExamples representing examples and the corresponding label.
    pub examples: &'input [ClassifyExample<'input>],
}

#[derive(strum_macros::Display, Serialize, Debug)]
pub enum ClassifyModel {
    #[strum(serialize = "small")]
    Small,
    #[strum(serialize = "large")]
    Large,
    #[strum(serialize = "multilingual-22-12")]
    Multilingual2212,
    Custom(String),
}

#[derive(Serialize, Debug)]
pub struct ClassifyExample<'input> {
    /// The text of the example.
    pub text: &'input str,
    /// The label that fits the example's text.
    pub label: &'input str,
}

#[derive(Deserialize, Debug)]
pub struct Confidence {
    /// The label.
    pub label: String,
    /// The associated confidence with the label.
    pub confidence: f64,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct LabelProperties {
    pub confidence: f64,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct Classification {
    pub id: String,
    /// The top predicted label for the text.
    pub prediction: String,
    /// Confidence score for the top predicted label.
    pub confidence: f32,
    /// Confidence score for each label.
    pub labels: HashMap<String, LabelProperties>,
    /// The text that is being classified.
    pub input: String,
}

#[derive(Deserialize, Debug)]
pub(crate) struct ClassifyResponse {
    pub classifications: Vec<Classification>,
}
