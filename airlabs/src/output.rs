use super::*;

mod airline;

#[derive(Debug, Args)]
pub(crate) struct OutputParams {
    #[arg(short, long, global = true)]
    pub debug: bool,
    #[arg(short, long, global = true)]
    pub raw: bool,
    #[arg(short, long, global = true)]
    pub json: bool,
}

pub(crate) trait Output {
    fn raw(&self) -> String;
    fn json(&self) -> json::Result<json::Value>;
    fn typed(&self) -> json::Result<String>;
}

impl<T> Output for Response<T>
where
    T: serde::de::DeserializeOwned + std::fmt::Debug,
{
    fn raw(&self) -> String {
        self.raw().to_string()
    }

    fn json(&self) -> json::Result<json::Value> {
        self.json()
    }

    fn typed(&self) -> json::Result<String> {
        let text = match self.response()?.into_result() {
            Ok(typed) => format!("{typed:?}"),
            Err(error) => error.message,
        };
        Ok(text)
    }
}
