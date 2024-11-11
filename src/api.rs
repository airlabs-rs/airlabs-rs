use super::*;

pub use airlines::Airline;
pub use airlines::AirlinesRequest;
pub use error::ErrorResponse;

mod airlines;
mod error;

fn default<T: Default>() -> T {
    T::default()
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub(crate) enum ApiResponse<T> {
    Success(T),
    Failure(ErrorResponse),
}

impl<T> ApiResponse<T> {
    pub fn into_result(self) -> Result<T, Error> {
        match self {
            Self::Success(success) => Ok(success),
            Self::Failure(error) => Err(Error::Failure(error)),
        }
    }
}

pub(crate) trait AirLabsRequest {
    fn url(&self, base: &str) -> String;
}
