use super::*;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Failure(#[from] api::Error),
    #[error(transparent)]
    Transport(#[from] reqwest::Error),
}
