use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

pub use airlines::Airline;
pub use airlines::AirlineFree;
pub use airlines::AirlinesRequest;
pub use airports::Airport;
pub use airports::AirportFree;
pub use airports::AirportsRequest;
pub use error::Error;
pub use flight::AircraftType;
pub use flight::Flight;
pub use flight::FlightFree;
pub use ping::PingRequest;
pub use ping::Pong;
pub use request::ClientInfo;
pub use request::ConnectionInfo;
pub use request::Karma;
pub use request::Key;
pub use request::Params;
pub use request::Request;

mod airlines;
mod airports;
mod error;
mod flight;
mod ping;
mod request;

#[derive(Debug, Serialize, Deserialize)]
pub struct Response<T> {
    pub request: Option<Request>,
    #[serde(flatten)]
    pub result: ApiResult<T>,
    pub terms: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ApiResult<T> {
    Response(T),
    Error(Error),
}

impl<T> Response<T> {
    pub fn into_result(self) -> Result<T, Error> {
        self.result.into_result()
    }

    pub fn request(&self) -> Option<&Request> {
        self.request.as_ref()
    }
}

impl<T> ApiResult<T> {
    pub fn into_result(self) -> Result<T, Error> {
        match self {
            Self::Response(value) => Ok(value),
            Self::Error(error) => Err(error),
        }
    }
}

pub trait AirLabsRequest: Sized {
    fn url(&self, base: &str) -> String;
}

fn default<T: Default>() -> T {
    T::default()
}
