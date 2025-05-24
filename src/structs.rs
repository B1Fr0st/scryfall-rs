#![allow(dead_code)]

use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Formatter;
use std::fmt::{self, Display};
use thiserror::Error;

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

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Colors {
    W,
    R,
    U,
    G,
    B,
}

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
    #[serde(rename = "split")]
    Split,
    #[serde(rename = "flip")]
    Flip,
    #[serde(rename = "transform")]
    Transform,
    #[serde(rename = "modal_dfc")]
    ModalDFC,
    #[serde(rename = "meld")]
    Meld,
    #[serde(rename = "leveler")]
    Leveler,
    #[serde(rename = "class")]
    Class,
    #[serde(rename = "case")]
    Case,
    #[serde(rename = "saga")]
    Saga,
    #[serde(rename = "adventure")]
    Adventure,
    #[serde(rename = "mutate")]
    Mutate,
    #[serde(rename = "prototype")]
    Prototype,
    #[serde(rename = "battle")]
    Battle,
    #[serde(rename = "planar")]
    Planar,
    #[serde(rename = "scheme")]
    Scheme,
    #[serde(rename = "vanguard")]
    Vanguard,
    #[serde(rename = "token")]
    Token,
    #[serde(rename = "double_faced_token")]
    DoubleFacedToken,
    #[serde(rename = "emblem")]
    Emblem,
    #[serde(rename = "augment")]
    Augment,
    #[serde(rename = "host")]
    Host,
    #[serde(rename = "art_series")]
    ArtSeries,
    #[serde(rename = "reversible_card")]
    ReversibleCard,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RelatedCardType {
    #[serde(rename = "token")]
    Token,
    #[serde(rename = "meld_result")]
    MeldResult,
    #[serde(rename = "meld_part")]
    MeldPart,
    #[serde(rename = "combo_piece")]
    ComboPiece,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RelatedCard {
    pub id: ScryfallID,
    /// always "related_card"
    pub object: String,
    pub component: RelatedCardType,
    pub name: String,
    pub type_line: String,
    pub uri: Url,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BorderColor {
    #[serde(rename = "black")]
    Black,
    #[serde(rename = "white")]
    White,
    #[serde(rename = "borderless")]
    Borderless,
    #[serde(rename = "yellow")]
    Yellow,
    #[serde(rename = "silver")]
    Silver,
    #[serde(rename = "gold")]
    Gold,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]

pub enum ImageType {
    #[serde(rename = "png")]
    Png,
    #[serde(rename = "border_crop")]
    BorderCrop,
    #[serde(rename = "art_crop")]
    ArtCrop,
    #[serde(rename = "large")]
    Large,
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "small")]
    Small,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Finishes {
    #[serde(rename = "nonfoil")]
    NonFoil,
    #[serde(rename = "foil")]
    Foil,
    #[serde(rename = "etched")]
    Etched,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FrameLayout {
    #[serde(rename = "1993")]
    Frame1993,
    #[serde(rename = "1997")]
    Frame1997,
    #[serde(rename = "2003")]
    Frame2003,
    #[serde(rename = "2015")]
    Frame2015,
    #[serde(rename = "future")]
    FrameFuture,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FrameEffect {
    #[serde(rename = "legendary")]
    Legendary,
    #[serde(rename = "miracle")]
    Miracle,
    #[serde(rename = "enchantment")]
    Enchantment,
    #[serde(rename = "draft")]
    Draft,
    #[serde(rename = "devoid")]
    Devoid,
    #[serde(rename = "tombstone")]
    Tombstone,
    #[serde(rename = "colorshifted")]
    Colorshifted,
    #[serde(rename = "inverted")]
    Inverted,
    #[serde(rename = "sunmoondfc")]
    SunMoonDFC,
    #[serde(rename = "compasslanddfc")]
    CompassLandDFC,
    #[serde(rename = "originpwdfc")]
    OriginPwDFC,
    #[serde(rename = "mooneldrazidfc")]
    MoonEldraziDFC,
    #[serde(rename = "waxingandwaningmoondfc")]
    WaxingAndWaningMoonDFC,
    #[serde(rename = "showcase")]
    Showcase,
    #[serde(rename = "extendedart")]
    ExtendedArt,
    #[serde(rename = "companion")]
    Companion,
    #[serde(rename = "etched")]
    Etched,
    #[serde(rename = "snow")]
    Snow,
    #[serde(rename = "lesson")]
    Lesson,
    #[serde(rename = "shatteredglass")]
    ShatteredGlass,
    #[serde(rename = "convertdfc")]
    ConvertDFC,
    #[serde(rename = "fandfc")]
    FanDFC,
    #[serde(rename = "upsidedowndfc")]
    UpsideDownDFC,
    #[serde(rename = "spree")]
    Spree,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Game {
    #[serde(rename = "paper")]
    Paper,
    #[serde(rename = "arena")]
    Arena,
    #[serde(rename = "mtgo")]
    Mtgo,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ImageStatus {
    #[serde(rename = "missing")]
    Missing,
    #[serde(rename = "placeholder")]
    Placeholder,
    #[serde(rename = "lowres")]
    LowRes,
    #[serde(rename = "highres_scan")]
    HighResScan,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PriceType {
    #[serde(rename = "usd")]
    USD,
    #[serde(rename = "usd_foil")]
    USDFoil,
    #[serde(rename = "usd_etched")]
    USDEtched,
    #[serde(rename = "eur")]
    EUR,
    #[serde(rename = "eur_foil")]
    EURFoil,
    #[serde(rename = "tix")]
    Tix,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PurchaseType {
    #[serde(rename = "tcgplayer")]
    TcgPlayer,
    #[serde(rename = "cardmarket")]
    CardMarket,
    #[serde(rename = "cardhoarder")]
    CardHoarder,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Rarity {
    #[serde(rename = "common")]
    Common,
    #[serde(rename = "uncommon")]
    Uncommon,
    #[serde(rename = "rare")]
    Rare,
    #[serde(rename = "special")]
    Special,
    #[serde(rename = "mythic")]
    Mythic,
    #[serde(rename = "bonus")]
    Bonus,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SecurityStamp {
    #[serde(rename = "oval")]
    Oval,
    #[serde(rename = "triangle")]
    Triangle,
    #[serde(rename = "acorn")]
    Acorn,
    #[serde(rename = "circle")]
    Circle,
    #[serde(rename = "arena")]
    Arena,
    #[serde(rename = "heart")]
    Heart,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SetType {
    Unimplemented,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Preview {
    #[serde(rename = "previewed_at")]
    pub previewed_at: String,
    #[serde(rename = "source_uri")]
    pub source_uri: String,
    #[serde(rename = "source")]
    pub source: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum RelatedUriType {
    #[serde(rename = "gatherer")]
    Gatherer,
    #[serde(rename = "tcgplayer_infinite_articles")]
    TcgPlayerInfiniteArticles,
    #[serde(rename = "tcgplayer_infinite_decks")]
    TcgPlayerInfiniteDecks,
    #[serde(rename = "edhrec")]
    EdhRec,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CardFace {
    pub artist: Option<String>,
    pub artist_id: Option<String>,
    pub cmc: Option<f32>,
    pub color_indicator: Option<Vec<Colors>>,
    pub colors: Option<Vec<Colors>>,
    pub defense: Option<String>,
    pub flavor_text: Option<String>,
    pub illustration_id: Option<String>,
    pub image_uris: Option<HashMap<ImageType, Url>>,
    pub layout: Option<Layout>,
    pub loyalty: Option<String>,
    pub mana_cost: Option<String>,
    pub name: String,
    /// always "card_face"
    pub object: String,
    pub oracle_id: Option<OracleID>,
    pub oracle_text: Option<String>,
    pub power: Option<String>,
    pub printed_name: Option<String>,
    pub printed_text: Option<String>,
    pub printed_type_line: Option<String>,
    pub toughness: Option<String>,
    pub type_line: Option<String>,
    pub watermark: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    pub all_parts: Option<Vec<RelatedCard>>,
    pub card_faces: Option<Vec<CardFace>>,
    pub cmc: Option<f32>,
    pub color_identity: Option<Vec<Colors>>,
    pub color_indicator: Option<Vec<Colors>>,
    pub colors: Option<Vec<Colors>>,
    pub defense: Option<String>,
    pub edhrec_rank: Option<i32>,
    pub game_changer: Option<bool>,
    pub hand_modifier: Option<String>,
    pub keywords: Vec<String>,
    pub legalities: Legalities,
    pub life_modifier: Option<String>,
    pub loyalty: Option<String>,
    pub mana_cost: Option<String>,
    pub name: String,
    pub oracle_text: Option<String>,
    pub penny_rank: Option<i32>,
    pub power: Option<String>,
    pub produced_mana: Option<Vec<String>>,
    pub reserved: bool,
    pub toughness: Option<String>,
    // API Docs say that this is not optional, but some cards don't have it based on our testing
    // including:
    // Grimgrin, Corpse-Born // Grimgrin, Corpse-Born
    pub type_line: Option<String>,

    // "Print Fields"
    pub artist: Option<String>,
    pub artist_ids: Option<Vec<String>>,
    pub booster: bool,
    pub border_color: BorderColor,
    //This is not an optional in Scryfall's API, but some cards don't have it based on our testing, including:
    // Arlinn Kord // Arlinn, Embraced by the Moon
    // Crystal Fragments // Summon: Alexander
    // seems to be transform cards
    pub card_back_id: Option<ScryfallID>,
    pub collector_number: String,
    pub content_warning: Option<bool>,
    pub digital: bool,
    pub finishes: Vec<Finishes>,
    pub flavor_name: Option<String>,
    pub flavor_text: Option<String>,
    pub flavor_effects: Option<Vec<FrameEffect>>,
    pub frame: FrameLayout,
    pub full_art: bool,
    pub games: Vec<Game>,
    pub highres_image: bool,
    pub illustration_id: Option<String>,
    pub image_status: ImageStatus,
    pub image_uris: Option<HashMap<ImageType, Url>>,
    pub oversized: bool,
    pub prices: HashMap<PriceType, Option<String>>,
    pub printed_name: Option<String>,
    pub printed_text: Option<String>,
    pub printed_type_line: Option<String>,
    pub promo: bool,
    pub promo_types: Option<Vec<String>>,
    pub purchase_uris: Option<HashMap<PurchaseType, Url>>,
    pub rarity: Rarity,
    pub related_uris: Option<HashMap<RelatedUriType, Url>>,
    pub released_at: String, //convert to date using external crate??
    pub reprint: bool,
    pub scryfall_set_uri: Url,
    pub set_name: String,
    pub set_search_uri: Url,
    pub set_type: String, //convert to SetType enum when we have comphrehensive list of set types
    pub set_uri: Url,
    pub set: String,
    pub set_id: String,
    pub story_spotlight: bool,
    pub textless: bool,
    pub variation: bool,
    pub variation_of: Option<String>, //could be replaced with ScryfallID?
    pub security_stamp: Option<SecurityStamp>,
    pub watermark: Option<String>,
    pub preview: Option<Preview>,
}
