use std::marker::PhantomData;
use std::time;

use super::*;

#[derive(Clone, Debug)]
pub struct RawResponse {
    duration: time::Duration,
    raw: String,
}

impl RawResponse {
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
        T: DeserializeOwned,
    {
        self.json().and_then(json::from_value)
    }

    pub fn api_response<T>(&self) -> json::Result<api::Response<T>>
    where
        T: DeserializeOwned,
    {
        self.typed()
    }
}

#[derive(Debug)]
pub enum ResponseType<T, U> {
    Regular(T),
    Free(U),
}

#[derive(Clone, Debug)]
pub struct Response<R> {
    raw: RawResponse,
    phantom: PhantomData<R>,
}

impl<R> Response<R>
where
    R: api::AirLabsRequest,
{
    pub fn from_raw(raw: RawResponse) -> Self {
        let phantom = PhantomData;
        Self { raw, phantom }
    }

    pub fn duration(&self) -> time::Duration {
        self.raw.duration()
    }

    pub fn raw(&self) -> &str {
        self.raw.raw()
    }

    pub fn json(&self) -> json::Result<json::Value> {
        self.raw.json()
    }

    pub fn is_free(&self) -> json::Result<bool> {
        let response = self.raw.json()?;
        let free = response["request"]["key"]["type"]
            .as_str()
            .is_some_and(|text| text == "free");
        Ok(free)
    }

    pub fn api_response(
        &self,
    ) -> json::Result<api::Response<ResponseType<R::Response, R::ResponseFree>>> {
        let response = self.raw.json()?;
        let free = response["request"]["key"]["type"]
            .as_str()
            .is_some_and(|text| text == "free");
        let response = if free {
            json::from_value::<api::Response<R::ResponseFree>>(response)?.map(ResponseType::Free)
        } else {
            json::from_value::<api::Response<R::Response>>(response)?.map(ResponseType::Regular)
        };
        Ok(response)
    }
}

#[cfg(test)]
mod tests;
