use super::*;

#[derive(Clone, Debug, Subcommand)]
pub(super) enum Command {
    #[command(visible_alias = "airlines")]
    Airline,
}

impl Command {
    pub(super) async fn exec(self, client: &Client, params: &OutputParams) -> anyhow::Result<()> {
        match self {
            Self::Airline => {
                let response = client.airlines_free().await?;
                self.show::<Vec<api::AirlineFree>>(response, params)?;
            }
        }
        Ok(())
    }

    fn show<T>(&self, response: Response, params: &OutputParams) -> json::Result<()>
    where
        T: serde::de::DeserializeOwned + Output,
    {
        if params.debug {
            todo!("debug")
        } else if params.raw {
            println!("{}", response.raw());
        } else if params.json {
            println!("{}", response.json()?)
        } else {
            let response = response.api_response::<T>()?;
            match response.into_result() {
                Ok(typed) => println!("{}", typed.output()),
                Err(err) => println!("{err}"),
            }
        }

        if params.stats {
            println!("API call took {:?}", response.duration());
        }

        Ok(())
    }
}
