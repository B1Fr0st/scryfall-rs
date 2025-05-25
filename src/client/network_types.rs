use crate::structs::Card;
use serde::Deserialize;
use thiserror::Error;

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum ScryfallResponse {
    #[serde(rename = "card")]
    Card(Box<Card>),
    #[serde(rename = "error")]
    Error(ScryfallError),
}

#[derive(Deserialize, Debug, Error)]
pub struct ScryfallError {
    pub status: u16,
    pub code: String,
    pub details: String,
    pub type_: Option<String>,
    pub warnings: Option<Vec<String>>,
}

impl std::fmt::Display for ScryfallError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Scryfall Error: {} - {}", self.code, self.details)
    }
}

pub trait ToScryfallError {
    fn to_scryfall_error(&self) -> ScryfallError;
}

impl ToScryfallError for reqwest::Error {
    fn to_scryfall_error(&self) -> ScryfallError {
        ScryfallError {
            status: 500,
            code: "reqwest_error".to_string(),
            details: self.to_string(),
            type_: None,
            warnings: None,
        }
    }
}
