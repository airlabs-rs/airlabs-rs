use super::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Pong {
    Pong,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PingRequest {
    pub api_key: String,
}

impl PingRequest {
    pub fn new(key: &str) -> Self {
        let api_key = key.into();
        Self { api_key }
    }
}

impl AirLabsRequest for PingRequest {
    type Response = Pong;
    type ResponseFree = Pong;
    const METHOD: &'static str = "ping";
}
