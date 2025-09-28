use my_assitant::{
    db::{create_shema, query_data},
    pdf::fetch_pdf,
};
use sqlx::{Connection, SqliteConnection};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let result = fetch().await?;
    let mut conn = SqliteConnection::connect("sqlite:data.db").await?;
    create_shema(&mut conn).await?;
    // for item in result.announcements {
    //     insert_data(&mut conn, item).await?;
    // }
    let urls = query_data(&mut conn).await?;
    for url in urls {
        println!("url is {url}");
        fetch_pdf(&url).await;
        break;
    }
    Ok(())
}
