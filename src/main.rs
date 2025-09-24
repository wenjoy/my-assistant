use my_assitant::{db::create_shema, fetch};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // fetch().await?;
    create_shema().await?;
    Ok(())
}
