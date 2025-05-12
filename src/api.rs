use super::*;

pub use airlines::Airline;
pub use airlines::AirlinesRequest;
pub use airports::Airport;
pub use airports::AirportsRequest;
pub use error::ErrorResponse;
pub use flight::Flight;
pub use flight::FlightQuery;
pub use flight::FlightRequest;

mod airlines;
mod airports;
mod error;
mod flight;

fn default<T: Default>() -> T {
    T::default()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseRequestInfo {}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseWrapper<T> {
    pub request: ResponseRequestInfo,
    pub response: T,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub(crate) enum ApiResponse<T> {
    Success(T),
    Failure(ErrorResponse),
}

impl<T> ApiResponse<T> {
    pub(crate) fn into_result(self) -> Result<T, Error> {
        match self {
            Self::Success(success) => Ok(success),
            Self::Failure(error) => Err(Error::Failure(error)),
        }
    }
}

pub(crate) trait AirLabsRequest {
    fn url(&self, base: &str) -> String;
}
