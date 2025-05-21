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

impl AirLabsRequest for PingRequest {
    fn url(&self, base: &str) -> String {
        format!("{base}/ping")
    }
}
