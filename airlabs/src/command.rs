use super::*;

#[derive(Clone, Debug, Subcommand)]
pub(super) enum Command {
    #[command(visible_alias = "airlines")]
    Airline,
}

impl Command {
    pub(super) async fn exec(self, client: &Client) -> anyhow::Result<Box<dyn Output>> {
        match self {
            Self::Airline => {
                let response = client.airlines_free().await?;
                Ok(Box::new(response))
            }
        }
    }
}
