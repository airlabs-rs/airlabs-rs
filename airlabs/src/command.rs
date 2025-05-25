use super::*;

#[derive(Clone, Debug, Subcommand)]
pub(super) enum Command {
    /// Get airline list
    #[command(visible_alias = "airlines")]
    Airline,

    /// Get airport list
    #[command(visible_alias = "airports")]
    Airport,

    /// Get flight information
    Flight(Flight),

    /// Ping API server
    Ping,
}

#[derive(Clone, Debug, Args)]
#[group(required = true, multiple = false)]
pub(super) struct Flight {
    /// IATA flight code
    #[arg(long)]
    iata: Option<String>,

    /// ICAO flight code
    #[arg(long)]
    icao: Option<String>,
}

impl Command {
    pub(super) async fn exec(self, client: &Client, params: &OutputParams) -> anyhow::Result<()> {
        match self {
            Self::Airline => {
                let response = client.airlines().await?;
                if client.is_free() {
                    self.show::<Vec<api::AirlineFree>>(response, params)?;
                } else {
                    self.show::<Vec<api::Airline>>(response, params)?;
                }
            }

            Self::Airport => {
                let response = client.airports().await?;
                if client.is_free() {
                    self.show::<Vec<api::AirportFree>>(response, params)?;
                } else {
                    self.show::<Vec<api::Airport>>(response, params)?;
                }
            }

            Self::Flight(ref flight) => {
                let response = if let Some(code) = flight.iata.as_ref() {
                    client.flight_iata(code).await?
                } else if let Some(code) = flight.icao.as_ref() {
                    client.flight_icao(code).await?
                } else {
                    unreachable!()
                };
                if client.is_free() {
                    self.show::<api::FlightFree>(response, params)?;
                } else {
                    self.show::<api::Flight>(response, params)?;
                }
            }
            Self::Ping => {
                let response = client.ping().await?;
                self.show::<api::Pong>(response, params)?;
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
            let duration = response.duration();
            let response = response.api_response::<T>()?;
            if let Some(request) = response.request {
                println!("{request:#?}");
            }
            println!("API call took {duration:?}");
        }

        Ok(())
    }
}
