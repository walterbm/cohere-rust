use serde::Serialize;

pub mod classify;
pub mod detect_language;
pub mod detokenize;
pub mod embed;
pub mod generate;
pub mod summarize;
pub mod tokenize;

#[derive(strum_macros::Display, Serialize, Debug)]
pub enum Truncate {
    #[strum(serialize = "NONE")]
    None,
    #[strum(serialize = "START")]
    Start,
    #[strum(serialize = "END")]
    End,
}
