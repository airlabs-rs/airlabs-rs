use std::marker::PhantomData;

use super::*;

#[derive(Clone, Debug)]
pub struct Response<T> {
    raw: String,
    phantom: PhantomData<T>,
}

impl<T> Response<T>
where
    T: serde::de::DeserializeOwned,
{
    pub fn from_raw(raw: String) -> Self {
        let phantom = PhantomData;
        Self { raw, phantom }
    }

    pub fn raw(&self) -> &str {
        &self.raw
    }

    pub fn json(&self) -> json::Result<json::Value> {
        json::from_str(&self.raw)
    }

    pub fn typed(&self) -> json::Result<T>
    where
        T: serde::de::DeserializeOwned,
    {
        json::from_str(&self.raw)
    }

    pub fn response(&self) -> json::Result<api::Response<T>> {
        json::from_str(&self.raw)
    }
}
