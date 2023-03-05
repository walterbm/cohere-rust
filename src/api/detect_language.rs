use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub(crate) struct DetectLanguageRequest {
    /// List of detected languages, one per text
    pub texts: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub(crate) struct DetectLanguageResponse {
    /// List of detected languages, one per text
    pub results: Vec<DetectLanguageResult>,
}

#[derive(Deserialize, Debug, PartialEq)]
pub struct DetectLanguageResult {
    /// Name of the language, e.g. "Spanish"
    pub language_name: String,
    /// Code of the language, e.g. "es"
    pub language_code: String,
}
