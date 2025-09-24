use my_assitant::fetch;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    fetch().await?;
    Ok(())
}
