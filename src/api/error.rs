use super::*;

#[derive(Debug, Serialize, Deserialize, thiserror::Error)]
#[error("{error:#?}")]
pub struct ErrorResponse {
    pub error: Error,
    pub terms: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    pub message: String,
    pub code: Code,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Code {
    UnknownApiKey,       //	Provided API Key is invalid.
    ExpiredApiKey,       //	The provided API key has expired.
    UnknownMethod,       //	Provided method is not supported.
    WrongParams,         //	Some parameters is wrong.
    NotFound,            //	Requested data was not found.
    MinuteLimitExceeded, //	The number of requests per minute has been exceeded.
    HourLimitExceeded,   //	The number of requests per hour has been exceeded.
    MonthLimitExceeded,  //	The number of requests per month has been exceeded.
    InternalError,       //	An internal error occurred.
}
