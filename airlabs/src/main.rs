use std::env;

use airlabs_client::Client;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let token = env::var("AIRLABS_API_TOKEN")?;
    let client = Client::new(token);
    let airlines = client
        .airlines_free()
        .await
        .inspect_err(|err| println!("{err:#?}"))?;
    println!("{airlines:#?}");
    Ok(())
}
