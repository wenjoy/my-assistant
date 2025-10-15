use my_assitant::{crawl, server::http_server};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //TODO: do this job regularly
    // crawl().await?;
    http_server().await;
    Ok(())
}
