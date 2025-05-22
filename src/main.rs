use reqwest::ClientBuilder;
use scryfall_rs::structs::Card;
#[tokio::main]
async fn main() {
    let client = ClientBuilder::new()
        .user_agent("scryfall-rs")
        .build()
        .unwrap();

    let response = client
        .get("https://api.scryfall.com/cards/named?exact=Black%20Lotus")
        .send()
        .await
        .unwrap();
    let _card: Card = response.json().await.unwrap();
}
