use my_assitant::{
    Announcement, Query, QueryParams,
    db::{
        clear_database, create_shema, initial_database, insert_data, query_data, query_latest_data,
    },
    fetch, fetch_all, fetch_latest_data,
    pdf::{fetch_pdf, read_pdf},
};
use sqlx::Row;
use sqlx::{Connection, SqliteConnection, sqlite::SqliteRow};
use time::{OffsetDateTime, macros::datetime};
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
    initial_database(&mut conn).await?;
    clear_database(&mut conn).await?;

    let latest_row = query_latest_data(&mut conn).await;
    let mut latest_date = 0;
    match latest_row {
        Ok(row) => {
            println!("latest row: {:?}", row.len());
            latest_date = row.try_get("announcement_time").unwrap();
        }
        Err(err) => {
            eprintln!("Error fetching latest data: {}", err);
        }
    }

    let result;
    if latest_date > 0 {
        result = fetch_latest_data(latest_date, None).await?;
    } else {
        result = fetch_all().await?;
    }

    for item in result.announcements {
        insert_data(&mut conn, item).await?;
    }
    // let urls = query_data(&mut conn).await?;
    // for url in urls {
    //     println!("url is {url}");
    //     fetch_pdf(&url).await;
    //     break;
    // }
    Ok(())
}
