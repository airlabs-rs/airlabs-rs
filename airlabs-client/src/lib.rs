use std::time;

use airlabs_api as api;
use serde_json as json;

pub use error::Error;
pub use response::Response;
pub use response::TypedResponse;

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

    pub fn update_from_ping(self, ping: Response) -> json::Result<Self> {
        let free_account = ping
            .api_response::<api::Pong>()?
            .request
            .is_some_and(|request| request.is_free());
        Ok(Self {
            free_account,
            ..self
        })
    }

    pub fn is_free(&self) -> bool {
        self.free_account
    }

    pub async fn ping(&self) -> reqwest::Result<Response> {
        let request = api::PingRequest {
            api_key: self.key.clone(),
        };
        self.post(request).await
    }

    pub async fn airlines(&self) -> reqwest::Result<Response> {
        let request = api::AirlinesRequest::new(&self.key);
        self.get(request).await
    }

    pub async fn airlines_free_old(&self) -> Result<Vec<api::AirlineFree>, Error> {
        let request = api::AirlinesRequest::new(&self.key);
        self.get_old(request).await
    }

    pub async fn airports(&self) -> Result<Vec<api::Airport>, Error> {
        let request = api::AirportsRequest::new(&self.key);
        self.get_old(request).await
    }

    fn get_request<R>(&self, request: R) -> reqwest::RequestBuilder
    where
        R: api::AirLabsRequest + serde::Serialize,
    {
        let url = self.url(&request);
        self.client.get(url).query(&request)
    }

    pub async fn get<R>(&self, request: R) -> reqwest::Result<Response>
    where
        R: api::AirLabsRequest + serde::Serialize,
    {
        let start = time::Instant::now();
        let request = self.get_request(request);
        request
            .send()
            .await?
            .error_for_status()?
            .text()
            .await
            .map(|raw| Response::new(raw, start.elapsed()))
    }

    fn post_request<R>(&self, request: R) -> reqwest::RequestBuilder
    where
        R: api::AirLabsRequest + serde::Serialize,
    {
        let url = self.url(&request);
        self.client.post(url).json(&request)
    }

    pub async fn post<R>(&self, request: R) -> reqwest::Result<Response>
    where
        R: api::AirLabsRequest + serde::Serialize,
    {
        let start = time::Instant::now();
        let request = self.post_request(request);
        request
            .send()
            .await?
            .error_for_status()?
            .text()
            .await
            .map(|raw| Response::new(raw, start.elapsed()))
    }

    async fn _get_typed<R, T>(&self, request: R) -> reqwest::Result<TypedResponse<T>>
    where
        R: api::AirLabsRequest + serde::Serialize,
        T: for<'de> serde::Deserialize<'de>,
    {
        self.get(request).await.map(TypedResponse::from_response)
    }

    async fn get_old<R, T>(&self, request: R) -> Result<T, Error>
    where
        R: api::AirLabsRequest + serde::Serialize,
        T: for<'de> serde::Deserialize<'de>,
    {
        let request = self.get_request(request);
        let response = request
            .send()
            .await?
            .error_for_status()?
            .json::<api::Response<T>>()
            .await?
            .into_result()?;
        Ok(response)
    }

    fn url<T>(&self, request: &T) -> String
    where
        T: api::AirLabsRequest,
    {
        request.url(&self.base)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request() {
        let client = Client::new("TOKEN");
        let request = api::AirlinesRequest::new("TOKEN");
        let request = client.get_request(request).build().unwrap();
        let url = request.url();
        assert_eq!(url.query().unwrap(), "api_key=TOKEN");
    }
}
