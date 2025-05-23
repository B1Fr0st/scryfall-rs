use core::panic;

#[tokio::test]
/// This test checks the functionality of the `card_named` function
/// by requesting a card by its name.
/// It expects a successful response from the Scryfall API.
async fn test_card_named() {
    use scryfall_rs::ScryfallClient;

    let mut client = ScryfallClient::new("scryfall-rs");
    let client_card = client.card_named("Black Lotus").await;
    let card = match client_card {
        Ok(card) => card,
        Err(err) => {
            eprintln!("Error: {}", err);
            panic!("Failed to fetch card named 'Black Lotus'");
        }
    };
    assert_eq!(card.name, "Black Lotus");
}

#[tokio::test]
/// This test checks the error handling of the `card_named` function
/// by requesting a card that does not exist.
/// It expects a 404 error response from the Scryfall API.
async fn test_card_named_error() {
    use scryfall_rs::ScryfallClient;

    let mut client = ScryfallClient::new("scryfall-rs");
    let client_card = client.card_named("Nonexistent Card").await;
    let err = match client_card {
        Ok(_) => {
            eprintln!("Expected an error, but got a card.");
            panic!("Successfully fetched a card that does not exist");
        }
        Err(err) => err,
    };
    assert_eq!(err.status, 404);
}

#[tokio::test]
/// This test checks the error handling of the `card_named` function
/// by requesting a card with an invalid name.
/// It expects a 404 error response from the Scryfall API.
async fn test_card_named_invalid_name() {
    use scryfall_rs::ScryfallClient;

    let mut client = ScryfallClient::new("scryfall-rs");
    let client_card = client.card_named("Invalid Name!@#").await;
    let err = match client_card {
        Ok(_) => {
            eprintln!("Expected an error, but got a card.");
            panic!("Successfully fetched a card with an invalid name");
        }
        Err(err) => err,
    };
    assert_eq!(err.status, 404);
}
#[tokio::test]
/// This test checks the encoding of the card name
/// by requesting a card with a name that contains special characters.
/// It expects a successful response from the Scryfall API.
async fn test_card_named_special_characters() {
    use scryfall_rs::ScryfallClient;

    let mut client = ScryfallClient::new("scryfall-rs");
    let client_card = client.card_named("Y'shtola, Night's Blessed").await;
    let card = match client_card {
        Ok(card) => card,
        Err(err) => {
            eprintln!("Error: {}", err);
            panic!("Failed to fetch card named Y'shtola, Night's Blessed");
        }
    };
    assert_eq!(card.name, "Y'shtola, Night's Blessed");
}

#[tokio::test]
async fn test_api_inconsistency_card_back_id() {
    use scryfall_rs::ScryfallClient;

    let mut client = ScryfallClient::new("scryfall-rs");
    let client_card = client
        .card_named("Arlinn Kord // Arlinn, Embraced by the Moon")
        .await;
    let card = match client_card {
        Ok(card) => card,
        Err(err) => {
            eprintln!("Error: {}", err);
            panic!("Failed to fetch card named Arlinn Kord // Arlinn, Embraced by the Moon");
        }
    };
    assert_eq!(card.name, "Arlinn Kord // Arlinn, Embraced by the Moon");
}

#[tokio::test]
async fn test_api_inconsistency_type_line() {
    use scryfall_rs::ScryfallClient;

    let mut client = ScryfallClient::new("scryfall-rs");
    let client_card = client
        .card_named("Grimgrin, Corpse-Born // Grimgrin, Corpse-Born")
        .await;
    let card = match client_card {
        Ok(card) => card,
        Err(err) => {
            eprintln!("Error: {}", err);
            panic!("Failed to fetch card named Grimgrin, Corpse-Born // Grimgrin, Corpse-Born");
        }
    };
    assert_eq!(card.name, "Grimgrin, Corpse-Born // Grimgrin, Corpse-Born");
}
