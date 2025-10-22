use my_assitant::{
    crawl,
    db::query_all_data,
    server::{HttpMethod, Router, http_server},
};
use sqlx::Row;
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
                let mut conn = SqliteConnection::connect("sqlite:data.db").await.unwrap();
                let data = query_all_data(&mut conn).await.unwrap();
                println!(
                    "data is {}",
                    data[0].try_get::<i64, _>("announcement_time").unwrap()
                );
                ()
            })
        }),
    };
    http_server(router).await;
    Ok(())
}
