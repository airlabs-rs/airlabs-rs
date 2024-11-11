#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = airlabs::Client::new("KAAA");
    let airlines = client.airlines().await?;
    println!("{airlines:#?}");
    Ok(())
}
