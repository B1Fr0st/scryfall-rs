use crate::structs::{Card, ScryfallError, ScryfallResponse, ToScryfallError};
use reqwest::ClientBuilder;

#[allow(dead_code)]
async fn debug_scryfall_response(response: reqwest::Response) -> ScryfallResponse {
    let json_str = response
        .text()
        .await
        .map_err(|e| e.to_scryfall_error())
        .unwrap();
    dbg!(&json_str);
    // parse the json string into ScryfallResponse
    let json: ScryfallResponse = serde_json::from_str(&json_str)
        .map_err(|e| ScryfallError {
            status: 500,
            code: "json_parse_error".to_string(),
            details: e.to_string(),
            type_: None,
            warnings: None,
        })
        .unwrap();
    json
}

pub struct RateLimit {
    pub limit: u128,
    pub last: std::time::Instant,
}

impl RateLimit {
    fn new() -> Self {
        RateLimit {
            limit: 50, //delay requested by Scryfall good citizenship
            last: std::time::Instant::now(),
        }
    }
    fn check(&mut self) {
        let now = std::time::Instant::now();
        if now.duration_since(self.last).as_millis() > self.limit {
            self.last = now;
        }
        let sleep = self.limit - now.duration_since(self.last).as_millis();
        if sleep > 0 {
            std::thread::sleep(std::time::Duration::from_millis(sleep as u64));
        }
    }
}

pub struct ScryfallClient {
    pub client: reqwest::Client,
    pub rate_limit: RateLimit,
}

impl ScryfallClient {
    pub fn new(user_agent: &str) -> Self {
        let client = ClientBuilder::new().user_agent(user_agent).build().unwrap();
        ScryfallClient {
            client,
            rate_limit: RateLimit::new(),
        }
    }

    pub async fn card_named(&mut self, name: &str) -> Result<Card, ScryfallError> {
        let url = format!(
            "https://api.scryfall.com/cards/named?exact={}",
            urlencoding::encode(name)
        );
        self.rate_limit.check();
        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| e.to_scryfall_error())?;

        let json: ScryfallResponse = response.json().await.map_err(|e| e.to_scryfall_error())?;
        match json {
            ScryfallResponse::Card(card) => Ok(*card),
            ScryfallResponse::Error(err) => Err(err),
        }
    }

    pub async fn card_random(&mut self) -> Result<Card, ScryfallError> {
        let url = "https://api.scryfall.com/cards/random";
        self.rate_limit.check();
        let response = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|e| e.to_scryfall_error())?;

        let json: ScryfallResponse = response.json().await.map_err(|e| e.to_scryfall_error())?;
        match json {
            ScryfallResponse::Card(card) => Ok(*card),
            ScryfallResponse::Error(err) => Err(err),
        }
    }
}
