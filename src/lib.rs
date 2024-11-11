use serde::{Deserialize, Serialize};

pub use error::Error;

mod api;
mod error;

#[derive(Debug)]
pub struct Client {
    base: String,
    client: reqwest::Client,
    key: String,
}

impl Client {
    pub fn new(key: impl ToString) -> Self {
        let base = "http://airlabs.co/api/v9".to_string();
        let key = key.to_string();
        let client = reqwest::Client::new();
        Self { base, client, key }
    }

    pub async fn airlines(&self) -> Result<api::Airline, Error> {
        let request = api::AirlinesRequest::new(&self.key);
        self.get(request).await
    }

    async fn get<R, T>(&self, request: R) -> Result<T, Error>
    where
        R: api::AirLabsRequest + serde::Serialize,
        T: for<'de> serde::Deserialize<'de>,
    {
        let url = self.url(&request);
        self.client
            .get(url)
            .query(&request)
            .send()
            .await?
            .error_for_status()?
            .json::<api::ApiResponse<T>>()
            .await?
            .into_result()
    }

    fn url<T>(&self, request: &T) -> String
    where
        T: api::AirLabsRequest,
    {
        request.url(&self.base)
    }
}
