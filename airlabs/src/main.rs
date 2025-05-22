use airlabs_api as api;
use airlabs_client::Client;
// use airlabs_client::Error;
use airlabs_client::Response;
use clap::Args;
use clap::Parser;
use clap::Subcommand;
use serde_json as json;

use command::Command;
use output::Output;
use output::OutputParams;

mod command;
mod output;

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(flatten)]
    params: OutputParams,

    /// Airlabs API token
    #[arg(long, global = true, env = "AIRLABS_API_TOKEN", hide_env_values = true)]
    pub token: Option<String>,

    #[command(subcommand)]
    command: Command,
}

impl Cli {
    async fn exec(self) -> anyhow::Result<()> {
        let token = self.token.as_deref().unwrap_or_default();
        let client = Client::new(token);
        let ping = client.ping().await?;
        let client = client.update_from_ping(ping)?;
        self.command.exec(&client, &self.params).await
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    Cli::parse().exec().await
}
