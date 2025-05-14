#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = airlabs::Client::new("KAAA");
    let airlines = client
        .airlines()
        .await
        .inspect_err(|err| println!("{err:#?}"))?;
    println!("{airlines:#?}");
    Ok(())
}
