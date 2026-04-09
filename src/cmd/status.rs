use crate::client::Client;
use crate::error::Result;

pub async fn execute(client: &Client) -> Result<()> {
    let resp = client.get("/health").await?;
    let body = resp.text().await?;
    println!("{body}");
    Ok(())
}
