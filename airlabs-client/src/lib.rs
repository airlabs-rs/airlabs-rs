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
    id: Option<u64>,
    free_account: bool,
}

impl Client {
    pub fn new(key: impl ToString) -> Self {
        let base = "https://airlabs.co/api/v9".to_string();
        let key = key.to_string();
        let id = None;
        let client = reqwest::Client::new();
        let free_account = false;
        Self {
            base,
            client,
            key,
            id,
            free_account,
        }
    }

    pub fn update_from_ping(self, ping: Response<api::PingRequest>) -> json::Result<Self> {
        let response = ping.api_response()?;
        let (id, free_account) = if let Some(request) = response.request() {
            let key = request.key();
            (Some(key.id), key.is_free())
        } else {
            (self.id, self.free_account)
        };
        Ok(Self {
            free_account,
            id,
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
        let signature = self.signature();
        let auth = if let Some(signature) = signature.as_ref() {
            &[("signature", signature)]
        } else {
            &[("api_key", &self.key)]
        };
        let url = self.url(&request);
        self.client.get(url).query(auth).query(&request)
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

    fn signature(&self) -> Option<String> {
        let api_id = self.id?;
        let timestamp = time::SystemTime::now()
            .duration_since(time::SystemTime::UNIX_EPOCH)
            .ok()?
            .as_secs();
        let hash = md5::compute(format!("{timestamp}:{api_key}", api_key = self.key));
        Some(format!("{api_id}:{timestamp}:{hash:x}"))
    }
}

#[cfg(test)]
mod tests;
