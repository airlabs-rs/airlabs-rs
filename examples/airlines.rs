use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let token = env::var("AIRLABS_API_TOKEN")?;
    let client = airlabs::Client::new(token);
    let airlines = client
        .airlines_free()
        .await
        .inspect_err(|err| println!("{err:#?}"))?;
    println!("{airlines:#?}");
    Ok(())
}
