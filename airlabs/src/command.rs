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
                let request = client.airlines();
                let response = client.get(request).await?;
                self.show(response, params)?;
            }

            Self::Airport => {
                let request = client.airports();
                let response = client.get(request).await?;
                self.show(response, params)?;
            }

            Self::Flight(ref flight) => {
                let request = if let Some(code) = flight.iata.as_ref() {
                    client.flight_iata(code)
                } else if let Some(code) = flight.icao.as_ref() {
                    client.flight_icao(code)
                } else {
                    unreachable!()
                };
                let response = client.get(request).await?;
                self.show(response, params)?;
            }

            Self::Ping => {
                let response = client.ping().await?;
                self.show(response, params)?;
            }
        }
        Ok(())
    }

    fn show<R>(&self, response: Response<R>, params: &OutputParams) -> json::Result<()>
    where
        R: api::AirLabsRequest,
        R::Response: Output,
        R::ResponseFree: Output,
    {
        if params.debug {
            todo!("debug")
        } else if params.raw {
            println!("{}", response.raw());
        } else if params.json {
            println!("{}", response.json()?)
        } else {
            let response = response.api_response()?;
            match response.into_result() {
                Ok(typed) => println!("{}", typed.output()),
                Err(err) => println!("{err}"),
            }
        }

        if params.stats {
            let duration = response.duration();
            let response = response.api_response()?;
            if let Some(request) = response.request {
                println!("{request:#?}");
            }
            println!("API call took {duration:?}");
        }

        Ok(())
    }
}
