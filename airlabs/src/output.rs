use itertools::Itertools;

use super::*;

mod airline;
mod airport;
mod flight;

#[derive(Debug, Args)]
pub(crate) struct OutputParams {
    /// Enable debug output
    #[arg(short, long, global = true)]
    pub debug: bool,

    /// Output raw API response
    #[arg(short, long, global = true)]
    pub raw: bool,

    /// Output API response as JSON
    #[arg(short, long, global = true)]
    pub json: bool,

    /// Also show API call stats
    #[arg(short, long, global = true)]
    pub stats: bool,
}

pub(crate) trait Output {
    fn output(&self) -> String;
}

impl<T> Output for Vec<T>
where
    T: Output,
{
    fn output(&self) -> String {
        self.iter().map(|item| item.output()).join("\n")
    }
}

impl Output for api::Pong {
    fn output(&self) -> String {
        format!("{self:?}")
    }
}

impl<T, U> Output for ResponseType<T, U>
where
    T: Output,
    U: Output,
{
    fn output(&self) -> String {
        match self {
            Self::Regular(item) => item.output(),
            Self::Free(item) => item.output(),
        }
    }
}
