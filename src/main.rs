use my_assitant::{
    crawl,
    server::{HttpMethod, Router, http_server},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //TODO: do this job regularly
    // crawl().await?;
    let router = Router {
        method: HttpMethod::GET,
        path: "/announcements".to_string(),
        handler: Box::new(|| {
            println!("handle router");
            ()
        }),
    };
    http_server(router).await;
    Ok(())
}
