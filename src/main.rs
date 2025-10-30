use my_assitant::{
    crawl,
    db::query_all_data,
    server::{HttpMethod, Router, http_server},
};
use sqlx::{Connection, SqliteConnection};
use tokio::time;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let router = Router {
        method: HttpMethod::GET,
        path: "/announcements".to_string(),
        handler: Box::new(|| {
            Box::pin(async {
                let mut conn = SqliteConnection::connect("sqlite:data.db").await.unwrap();
                let data = query_all_data(&mut conn).await.unwrap();
                // println!("data is {:?}", data);
                data
            })
        }),
    };
    http_server(router).await;
    let day1 = time::Duration::from_secs(24 * 60 * 60);
    let mut interval = time::interval(day1);
    loop {
        interval.tick().await;
        if let Err(e) = crawl().await {
            println!("Crawl error: {e}");
        }
    }
}
