#[tokio::test]
/// This test checks the error handling of the `card_random` function
/// by requesting a random card.
/// It expects a successful response from the Scryfall API.
async fn test_card_random() {
    use reqwest::ClientBuilder;
    use scryfall_rs::structs::Card;

    let client = ClientBuilder::new()
        .user_agent("scryfall-rs")
        .build()
        .expect("Failed to build client");
    let url = "https://api.scryfall.com/cards/random";
    let response = client
        .get(url)
        .send()
        .await
        .expect("Failed to send request");
    let json_str = response.text().await.expect("Failed to read response");
    let card: Card = serde_json::from_str(&json_str)
        .map_err(|e| {
            eprintln!("Failed to parse JSON: {}", e);
            dbg!(&json_str);
            panic!("Failed to parse JSON");
        })
        .expect("Failed to parse JSON");
    assert!(card.name.len() > 0);
}

#[tokio::test]
/// This test checks the error handling of the `card_random` function
/// by requesting multiple random cards.
/// It expects a successful response from the Scryfall API.
async fn test_card_random_multiple() {
    use reqwest::ClientBuilder;
    use scryfall_rs::structs::Card;

    let client = ClientBuilder::new()
        .user_agent("scryfall-rs")
        .build()
        .expect("Failed to build client");
    let url = "https://api.scryfall.com/cards/random";

    for iter in 0..20 {
        for _ in 0..10 {
            let response = client
                .get(url)
                .send()
                .await
                .expect("Failed to send request");
            let json_str = response.text().await.expect("Failed to read response");
            let card: Card = serde_json::from_str(&json_str)
                .map_err(|e| {
                    eprintln!("Failed to parse JSON: {}", e);
                    dbg!(&json_str);
                    panic!("Failed to parse JSON");
                })
                .expect("Failed to parse JSON");
            assert!(card.name.len() > 0);
            std::thread::sleep(std::time::Duration::from_millis(50)); //ratelimiting
        }
        println!("{} cards fetched", iter * 10 + 10);
    }
}

#[tokio::test]
#[ignore = "Highly expensive version of test_card_random_multiple"]
/// This test checks the error handling of the `card_random` function
/// by requesting multiple random cards.
/// It expects a successful response from the Scryfall API.
/// Expected to take at minimum 8.3 minutes to run solely due to rate limiting.
/// However, network latency and other factors will increase this time.
async fn test_card_random_multiple_10000() {
    use reqwest::ClientBuilder;
    use scryfall_rs::structs::Card;

    let client = ClientBuilder::new()
        .user_agent("scryfall-rs")
        .build()
        .expect("Failed to build client");
    let url = "https://api.scryfall.com/cards/random";

    for iter in 0..1000 {
        for _ in 0..10 {
            let response = client
                .get(url)
                .send()
                .await
                .expect("Failed to send request");
            let json_str = response.text().await.expect("Failed to read response");
            let card: Card = serde_json::from_str(&json_str)
                .map_err(|e| {
                    eprintln!("Failed to parse JSON: {}", e);
                    dbg!(&json_str);
                    panic!("Failed to parse JSON");
                })
                .expect("Failed to parse JSON");
            assert!(card.name.len() > 0);
            std::thread::sleep(std::time::Duration::from_millis(50)); //ratelimiting
        }
        println!("{} cards fetched", iter * 10 + 10);
    }
}
