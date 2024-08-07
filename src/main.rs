use rust_ftps::FtpsClient;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = FtpsClient::connect("ftps.google.com:34").await?;
    client.login("ayimdomnic", "password").await?;
    Ok(())
}