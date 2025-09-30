use my_assitant::{
    Announcement,
    db::{create_shema, insert_data, query_data},
    fetch,
    pdf::{fetch_pdf, read_pdf},
};
use sqlx::Row;
use sqlx::{Connection, SqliteConnection, sqlite::SqliteRow};
fn get_pdf_urls(data: Vec<SqliteRow>) -> Vec<String> {
    let pdf_urls = data
        .iter()
        .map(|item| item.try_get::<String, _>("adjunct_url").unwrap())
        .collect();

    pdf_urls
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut conn = SqliteConnection::connect("sqlite:data.db").await?;
    create_shema(&mut conn).await?;
    query_data(&mut conn).await;
    // let result = fetch().await?;
    // for item in result.announcements {
    //     insert_data(&mut conn, item).await?;
    // }
    // let urls = query_data(&mut conn).await?;
    // for url in urls {
    //     println!("url is {url}");
    //     fetch_pdf(&url).await;
    //     break;
    // }
    Ok(())
}
