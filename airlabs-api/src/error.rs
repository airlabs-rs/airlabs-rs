use super::*;

#[derive(Debug, Serialize, Deserialize, thiserror::Error)]
#[error("{message}")]
pub struct Error {
    pub message: String,
    pub code: Code,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Code {
    /// Provided API Key is invalid.
    UnknownApiKey,
    /// The provided API key has expired.
    ExpiredApiKey,
    /// Provided method is not supported.
    UnknownMethod,
    /// Some parameters is wrong.
    WrongParams,
    /// Requested data was not found.
    NotFound,
    /// The number of requests per minute has been exceeded.
    MinuteLimitExceeded,
    /// The number of requests per hour has been exceeded.
    HourLimitExceeded,
    /// The number of requests per month has been exceeded.
    MonthLimitExceeded,
    /// An internal error occurred.
    InternalError,
}
