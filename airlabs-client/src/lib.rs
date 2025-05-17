use airlabs_api as api;

pub use error::Error;

mod error;

#[derive(Debug)]
pub struct Client {
    base: String,
    client: reqwest::Client,
    key: String,
}

impl Client {
    pub fn new(key: impl ToString) -> Self {
        let base = "https://airlabs.co/api/v9".to_string();
        let key = key.to_string();
        let client = reqwest::Client::new();
        Self { base, client, key }
    }

    pub async fn airlines(&self) -> Result<Vec<api::Airline>, Error> {
        let request = api::AirlinesRequest::new(&self.key);
        self.get(request).await
    }

    pub async fn airlines_free(&self) -> Result<Vec<api::AirlineFree>, Error> {
        let request = api::AirlinesRequest::new(&self.key);
        self.get(request).await
    }

    pub async fn airports(&self) -> Result<Vec<api::Airport>, Error> {
        let request = api::AirportsRequest::new(&self.key);
        self.get(request).await
    }

    fn get_request<R>(&self, request: R) -> reqwest::RequestBuilder
    where
        R: api::AirLabsRequest + serde::Serialize,
    {
        let url = self.url(&request);
        self.client.get(url).query(&request)
    }

    async fn get<R, T>(&self, request: R) -> Result<T, Error>
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
