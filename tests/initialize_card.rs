use scryfall_rs::structs::{Card, Language, Layout, OracleID, ScryfallID};
use std::string::ToString;
use url::Url;

// tries to initialize a card with dummy data
#[test]
fn test_initialize_card() {
    let card = Card {
        arena_id: Some(123),
        id: ScryfallID("dummy-scryfall-id".to_string()),
        lang: Language::English,
        mtgo_id: Some(456),
        mtgo_foil_id: Some(789),
        multiverse_ids: vec![111, 222, 333],
        tcgplayer_id: Some(101112),
        tcgplayer_etched_id: Some(131415),
        cardmarket_id: Some(161718),
        object: "card".to_string(),
        layout: Layout::Normal, // You may need to adjust this if Layout has variants/fields
        oracle_id: Some(OracleID("dummy-oracle-id".to_string())),
        prints_search_uri: Url::parse("https://scryfall.com/card/prints").unwrap(),
        rulings_uri: Url::parse("https://scryfall.com/card/rulings").unwrap(),
        scryfall_uri: Url::parse("https://scryfall.com/card").unwrap(),
        uri: Url::parse("https://api.scryfall.com/card").unwrap(),
    };

    // Just assert that the card fields are as expected
    assert_eq!(card.object, "card");
    assert_eq!(card.lang, Language::English);
    assert_eq!(card.multiverse_ids.len(), 3);
    assert_eq!(card.uri.to_string(), "https://api.scryfall.com/card");
}
