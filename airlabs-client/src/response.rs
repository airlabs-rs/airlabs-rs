use std::marker::PhantomData;
use std::time;

use super::*;

#[derive(Clone, Debug)]
pub struct Response {
    duration: time::Duration,
    raw: String,
}

impl Response {
    pub fn new(raw: impl ToString, duration: time::Duration) -> Self {
        let raw = raw.to_string();
        Self { duration, raw }
    }

    pub fn duration(&self) -> time::Duration {
        self.duration
    }

    pub fn raw(&self) -> &str {
        &self.raw
    }

    pub fn json(&self) -> json::Result<json::Value> {
        json::from_str(&self.raw)
    }

    pub fn typed<T>(&self) -> json::Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        self.json().and_then(json::from_value)
    }

    pub fn api_response<T>(&self) -> json::Result<api::Response<T>>
    where
        T: serde::de::DeserializeOwned,
    {
        self.typed()
    }
}

#[derive(Clone, Debug)]
pub struct TypedResponse<T> {
    response: Response,
    phantom: PhantomData<T>,
}

impl<T> TypedResponse<T>
where
    T: serde::de::DeserializeOwned,
{
    pub fn from_response(response: Response) -> Self {
        let phantom = PhantomData;
        Self { response, phantom }
    }

    pub fn response(&self) -> json::Result<api::Response<T>> {
        self.response.typed()
    }
}

#[cfg(test)]
mod tests;
