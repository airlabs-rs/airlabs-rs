#![cfg_attr(feature = "pedantic", warn(clippy::pedantic))]
#![warn(clippy::use_self)]
#![warn(clippy::map_flatten)]
#![warn(clippy::map_unwrap_or)]
#![warn(deprecated_in_future)]
#![warn(future_incompatible)]
#![warn(noop_method_call)]
#![warn(unreachable_pub)]
#![warn(missing_debug_implementations)]
#![warn(rust_2018_compatibility)]
#![warn(rust_2021_compatibility)]
#![warn(rust_2024_compatibility)]
#![warn(rust_2018_idioms)]
#![warn(unused)]
#![deny(warnings)]

use std::collections::BTreeMap;

use api::FlightQuery;
use serde::{Deserialize, Serialize};

pub use error::Error;

pub mod api;
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

    pub async fn airlines(&self) -> Result<Vec<api::Airline>, Error> {
        let request = api::AirlinesRequest::new(&self.key);
        self.get(request).await
    }

    pub async fn airports(&self) -> Result<Vec<api::Airport>, Error> {
        let request = api::AirportsRequest::new(&self.key);
        self.get(request).await
    }

    pub async fn flight(&self, query: FlightQuery) -> Result<api::Flight, Error> {
        let request = api::FlightRequest::new(&self.key, query);
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
