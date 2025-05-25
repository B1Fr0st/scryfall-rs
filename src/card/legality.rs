use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Legality {
    #[serde(rename = "not_legal")]
    NotLegal,
    #[serde(rename = "legal")]
    Legal,
    #[serde(rename = "banned")]
    Banned,
    #[serde(rename = "restricted")]
    Restricted,
}

impl Display for Legality {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Legality::NotLegal => write!(f, "Not Legal"),
            Legality::Legal => write!(f, "Legal"),
            Legality::Banned => write!(f, "Banned"),
            Legality::Restricted => write!(f, "Restricted"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
