use std::marker::PhantomData;
use std::time;

use super::*;

#[derive(Debug)]
pub enum ResponseType<T, U> {
    Regular(T),
    Free(U),
}

impl<T, U> ResponseType<T, U> {
    pub fn regular(self) -> Option<T> {
        match self {
            Self::Regular(value) => Some(value),
            Self::Free(_) => None,
        }
    }

    pub fn free(self) -> Option<U> {
        match self {
            Self::Regular(_) => None,
            Self::Free(value) => Some(value),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Response<R> {
    raw: String,
    duration: time::Duration,
    phantom: PhantomData<R>,
}

impl<R> Response<R>
where
    R: api::AirLabsRequest,
{
    pub fn new(raw: impl ToString, duration: time::Duration) -> Self {
        let raw = raw.to_string();
        let phantom = PhantomData;
        Self {
            raw,
            duration,
            phantom,
        }
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

    pub fn is_free(&self) -> json::Result<bool> {
        let response = self.json()?;
        let free = response["request"]["key"]["type"]
            .as_str()
            .is_some_and(|text| text == "free");
        Ok(free)
    }

    pub fn api_response(
        &self,
    ) -> json::Result<api::Response<ResponseType<R::Response, R::ResponseFree>>> {
        let response = self.json()?;
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
