use super::*;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Failure(api::ErrorResponse),
    #[error(transparent)]
    Transport(reqwest::Error),
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Self::Transport(error)
    }
}
