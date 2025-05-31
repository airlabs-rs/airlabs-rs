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

#[derive(Debug)]
pub enum ResponseType<T, U> {
    Regular(T),
    Free(U),
}

#[derive(Clone, Debug)]
pub struct TypedResponse<R> {
    response: Response,
    phantom: PhantomData<R>,
}

impl<R> TypedResponse<R>
where
    R: api::AirLabsRequest,
    // T: serde::de::DeserializeOwned,
{
    pub fn from_response(response: Response) -> Self {
        let phantom = PhantomData;
        Self { response, phantom }
    }

    pub fn response(
        &self,
    ) -> json::Result<api::Response<ResponseType<R::Response, R::ResponseFree>>> {
        let response = self.response.json()?;
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
