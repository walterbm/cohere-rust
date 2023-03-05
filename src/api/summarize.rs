use serde::{Deserialize, Serialize};

#[derive(Serialize, Default, Debug)]
pub struct SummarizeRequest {
    /// Text to summarize
    pub text: String,
    /// 'One of `paragraph` or `bullets`, defaults to `paragraph`.
    /// Indicates the style in which the summary will be delivered - in a free form
    /// paragraph or in bullet points.'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<SummarizeFormat>,
    /// One of `short`, `medium` or `long`, defaults to `medium`. Indicates the approximate length of the summary.'
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length: Option<SummarizeLength>,
    /// One of `low`, `medium` or `high`, defaults to `low`. Controls how close to the original text the summary is.
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
    Medium,
    #[strum(serialize = "summarize-xlarge")]
    XLarge,
}

#[derive(strum_macros::Display, Serialize, Debug)]
pub enum SummarizeLength {
    #[strum(serialize = "short")]
    Short,
    #[strum(serialize = "medium")]
    Medium,
    #[strum(serialize = "long")]
    Long,
}

#[derive(strum_macros::Display, Serialize, Debug)]
pub enum SummarizeFormat {
    #[strum(serialize = "paragraph")]
    Paragraph,
    #[strum(serialize = "bullets")]
    Bullets,
}

#[derive(strum_macros::Display, Serialize, Debug)]
pub enum SummarizeExtractiveness {
    #[strum(serialize = "low")]
    Low,
    #[strum(serialize = "medium")]
    Medium,
    #[strum(serialize = "high")]
    High,
}

#[derive(Deserialize, Debug)]
pub(crate) struct SummarizeResponse {
    pub summary: String,
}
