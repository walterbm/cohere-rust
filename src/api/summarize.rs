use serde::{Deserialize, Serialize};

#[derive(Serialize, Default, Debug)]
pub struct SummarizeRequest<'input> {
    /// Text to summarize
    pub text: &'input str,
    /// 'One of `paragraph`, `bullets` or `auto`, defaults to `auto`.
    /// Indicates the style in which the summary will be delivered - in a free form
    /// paragraph or in bullet points.'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<SummarizeFormat>,
    /// One of `short`, `medium`, `long` or `auto` defaults to `auto`. Indicates the approximate length of the summary.'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<SummarizeLength>,
    /// One of `low`, `medium`, `high` or `auto`, defaults to `auto`. Controls how close to the original text the summary is.
    /// `high` extractiveness summaries will lean towards reusing sentences verbatim, while `low` extractiveness
    /// summaries will tend to paraphrase more.'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extractiveness: Option<SummarizeExtractiveness>,
    /// Ranges from 0 to 5. Controls the randomness of the output. Lower values tend to generate more “predictable” output,
    /// while higher values tend to generate more “creative” output. The sweet spot is typically between 0 and 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    /// A free-form instruction for modifying how the summaries get generated. Should complete the sentence "Generate a summary _".
    /// Eg. "focusing on the next steps" or "written by Yoda"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_command: Option<String>,
    /// Denotes the summarization model to be used. Defaults to the best performing model
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<SummarizeModel>,
}

#[derive(strum_macros::Display, Serialize, Debug)]
pub enum SummarizeModel {
    #[strum(serialize = "summarize-medium")]
    #[serde(rename = "summarize-medium")]
    Medium,
    #[strum(serialize = "summarize-xlarge")]
    #[serde(rename = "summarize-xlarge")]
    XLarge,
}

#[derive(strum_macros::Display, Serialize, Debug)]
pub enum SummarizeLength {
    #[strum(serialize = "short")]
    #[serde(rename = "short")]
    Short,
    #[strum(serialize = "medium")]
    #[serde(rename = "medium")]
    Medium,
    #[strum(serialize = "long")]
    #[serde(rename = "long")]
    Long,
    #[strum(serialize = "auto")]
    #[serde(rename = "auto")]
    Auto,
}

#[derive(strum_macros::Display, Serialize, Debug)]
pub enum SummarizeFormat {
    #[strum(serialize = "paragraph")]
    #[serde(rename = "paragraph")]
    Paragraph,
    #[strum(serialize = "bullets")]
    #[serde(rename = "bullets")]
    Bullets,
    #[strum(serialize = "auto")]
    #[serde(rename = "auto")]
    Auto,
}

#[derive(strum_macros::Display, Serialize, Debug)]
pub enum SummarizeExtractiveness {
    #[strum(serialize = "low")]
    #[serde(rename = "low")]
    Low,
    #[strum(serialize = "medium")]
    #[serde(rename = "medium")]
    Medium,
    #[strum(serialize = "high")]
    #[serde(rename = "high")]
    High,
    #[strum(serialize = "auto")]
    #[serde(rename = "auto")]
    Auto,
}

#[derive(Deserialize, Debug)]
pub(crate) struct SummarizeResponse {
    pub summary: String,
}
