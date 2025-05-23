use scryfall_rs::ScryfallClient;

#[tokio::main]
async fn main() {
    let mut client = ScryfallClient::new("scryfall-rs");
    let client_card = client.card_named("Grimgrin, Corpse-Born").await;
    let card = match client_card {
        Ok(card) => card,
        Err(err) => {
            eprintln!("Error: {}", err);
            return;
        }
    };
    dbg!(card);
}
