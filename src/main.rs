use my_assitant::crawl;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //TODO: do this job regularly
    crawl().await?;
    Ok(())
}
