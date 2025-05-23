use scryfall_rs::Card;

#[tokio::main]
async fn main() {
    let client = scryfall_rs::ScryfallClient::new("scryfall-rs");

    let response = client
        .client
        .get("https://api.scryfall.com/cards/named?exact=Black%20Lotus")
        .send()
        .await
        .unwrap();
    let card: Card = response.json().await.unwrap();
    dbg!(card);
}
