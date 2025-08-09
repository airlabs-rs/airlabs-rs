use std::time;

use airlabs_api as api;
use serde_json as json;

pub use error::Error;
pub use response::Response;
pub use response::ResponseType;

mod error;
mod response;

#[derive(Debug)]
pub struct Client {
    base: String,
    client: reqwest::Client,
    key: String,
    free_account: bool,
}

impl Client {
    pub fn new(key: impl ToString) -> Self {
        let base = "https://airlabs.co/api/v9".to_string();
        let key = key.to_string();
        let client = reqwest::Client::new();
        let free_account = false;
        Self {
            base,
            client,
            key,
            free_account,
        }
    }

    pub fn update_from_ping(self, ping: Response<api::PingRequest>) -> json::Result<Self> {
        let free_account = ping.is_free()?;
        Ok(Self {
            free_account,
            ..self
        })
    }

    pub fn is_free(&self) -> bool {
        self.free_account
    }

    pub fn airlines(&self) -> api::AirlinesRequest {
        api::AirlinesRequest::new()
    }

    pub fn airports(&self) -> api::AirportsRequest {
        api::AirportsRequest::new()
    }

    pub fn flight_iata(&self, code: impl ToString) -> api::FlightRequest {
        api::FlightRequest::iata(code)
    }

    pub fn flight_icao(&self, code: impl ToString) -> api::FlightRequest {
        api::FlightRequest::icao(code)
    }

    pub async fn ping(&self) -> reqwest::Result<Response<api::PingRequest>> {
        let request = api::PingRequest::new(&self.key);
        self.post(request).await
    }

    pub async fn get<R>(&self, request: R) -> reqwest::Result<Response<R>>
    where
        R: api::AirLabsRequest,
    {
        let start = time::Instant::now();
        self.get_request(request)
            .send()
            .await?
            .error_for_status()?
            .text()
            .await
            .map(|raw| Response::new(raw, start.elapsed()))
    }

    pub async fn post<R>(&self, request: R) -> reqwest::Result<Response<R>>
    where
        R: api::AirLabsRequest,
    {
        let start = time::Instant::now();
        self.post_request(request)
            .send()
            .await?
            .error_for_status()?
            .text()
            .await
            .map(|raw| Response::new(raw, start.elapsed()))
    }

    pub fn get_request<R>(&self, request: R) -> reqwest::RequestBuilder
    where
        R: api::AirLabsRequest,
    {
        let url = self.url(&request);
        self.client
            .get(url)
            .query(&[("api_key", &self.key)])
            .query(&request)
    }

    fn post_request<R>(&self, request: R) -> reqwest::RequestBuilder
    where
        R: api::AirLabsRequest,
    {
        let url = self.url(&request);
        self.client.post(url).json(&request)
    }

    fn url<T>(&self, request: &T) -> String
    where
        T: api::AirLabsRequest,
    {
        request.url(&self.base)
    }
}

#[cfg(test)]
mod tests;
