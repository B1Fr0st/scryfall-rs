#![allow(dead_code)]

use serde_derive::{Deserialize, Serialize};
use std::fmt::Formatter;
use std::fmt::{self, Display};

use url::Url;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ScryfallID(pub String);

impl Display for ScryfallID {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct OracleID(pub String);

impl Display for OracleID {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub enum Colors {
    W,
    R,
    U,
    G,
    B,
}

pub enum Legality {
    NotLegal,
    Legal,
}

impl Display for Legality {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Legality::NotLegal => write!(f, "Not Legal"),
            Legality::Legal => write!(f, "Legal"),
        }
    }
}

pub struct Legalities {
    //convert to rust enums
    standard: Legality,
    future: Legality,
    historic: Legality,
    timeless: Legality,
    gladiator: Legality,
    pioneer: Legality,
    explorer: Legality,
    modern: Legality,
    legacy: Legality,
    pauper: Legality,
    vintage: Legality,
    penny: Legality,
    commander: Legality,
    oathbreaker: Legality,
    standardbrawl: Legality,
    brawl: Legality,
    alchemy: Legality,
    paupercommander: Legality,
    duel: Legality,
    oldschool: Legality,
    premodern: Legality,
    predh: Legality,
}

impl Display for Legalities {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "Standard: {}, Modern: {}, Legacy: {}, Vintage: {}, Pauper: {}, Commander: {}, Brawl: {}, Duel: {}",
            self.standard,
            self.modern,
            self.legacy,
            self.vintage,
            self.pauper,
            self.commander,
            self.brawl,
            self.duel
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Language {
    #[serde(rename = "en")]
    English,
    #[serde(rename = "es")]
    Spanish,
    #[serde(rename = "fr")]
    French,
    #[serde(rename = "de")]
    German,
    #[serde(rename = "it")]
    Italian,
    #[serde(rename = "pt")]
    Portuguese,
    #[serde(rename = "ja")]
    Japanese,
    #[serde(rename = "ko")]
    Korean,
    #[serde(rename = "ru")]
    Russian,
    #[serde(rename = "zhs")]
    SimplifiedChinese,
    #[serde(rename = "zht")]
    TraditionalChinese,
    #[serde(rename = "he")]
    Hebrew,
    #[serde(rename = "la")]
    Latin,
    #[serde(rename = "grc")]
    AncientGreek,
    #[serde(rename = "ar")]
    Arabic,
    #[serde(rename = "sa")]
    Sanskrit,
    #[serde(rename = "ph")]
    Phyrexian,
    #[serde(rename = "qya")]
    Quenya,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Layout {
    #[serde(rename = "normal")]
    Normal,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Card {
    // "Core Card Fields"
    pub arena_id: Option<i32>,
    pub id: ScryfallID,
    pub lang: Language,
    pub mtgo_id: Option<i32>,
    pub mtgo_foil_id: Option<i32>,
    pub multiverse_ids: Vec<i32>,
    pub tcgplayer_id: Option<i32>,
    pub tcgplayer_etched_id: Option<i32>,
    pub cardmarket_id: Option<i32>,
    /// always "card"
    pub object: String,
    pub layout: Layout,
    /// only null when layout is "reversible_card"
    pub oracle_id: Option<OracleID>,
    pub prints_search_uri: Url,
    pub rulings_uri: Url,
    /// link to the card on scryfall's website
    pub scryfall_uri: Url,
    /// link to the card on scryfall's API
    pub uri: Url,
    // "Gameplay Fields"

    // "Print Fields"
}
