use reqwest::ClientBuilder;

pub struct ScryfallClient {
    pub client: reqwest::Client,
}

impl ScryfallClient {
    pub fn new(user_agent: &str) -> Self {
        let client = ClientBuilder::new().user_agent(user_agent).build().unwrap();
        ScryfallClient { client }
    }
}
