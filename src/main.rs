use my_assitant::{
    crawl,
    db::query_all_data,
    server::{HttpMethod, Router, http_server},
};
use sqlx::{Connection, SqliteConnection};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //TODO: do this job regularly
    // crawl().await?;
    let router = Router {
        method: HttpMethod::GET,
        path: "/announcements".to_string(),
        handler: Box::new(|| {
            Box::pin(async {
                println!("hello");
                // let mut conn = SqliteConnection::connect("sqlite::data.db").await?;
                // let data = query_all_data(&mut conn).await?;
                // Ok(())
            })
        }),
    };
    http_server(router).await;
    Ok(())
}
