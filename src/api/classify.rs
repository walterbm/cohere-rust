use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{EmbedModel, Truncate};

#[derive(Serialize, Default, Debug)]
pub struct ClassifyRequest<'input> {
    /// An optional string representing the model you'd like to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<EmbedModel>,
    /// An optional string representing the ID of a custom playground preset.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preset: Option<String>,
    /// An array of strings that you would like to classify.
    pub inputs: &'input [String],
    /// An array of ClassifyExamples representing examples and the corresponding label.
    pub examples: &'input [ClassifyExample<'input>],
    /// Specify how the API will handle inputs longer than the maximum token length.
    pub truncate: Option<Truncate>,
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
