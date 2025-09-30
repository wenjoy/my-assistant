use my_assitant::{
    Announcement, Query, QueryParams,
    db::{create_shema, insert_data, query_data, query_latest_data},
    fetch,
    pdf::{fetch_pdf, read_pdf},
};
use sqlx::Row;
use sqlx::{Connection, SqliteConnection, sqlite::SqliteRow};
use time::OffsetDateTime;
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
    let res = query_latest_data(&mut conn).await?;
    let latest_date = res.try_get::<i64, _>("announcement_time").unwrap();
    let latest_date = OffsetDateTime::from_unix_timestamp_nanos(latest_date as i128).unwrap();
    let zhe_shuang_bank_code = "601916,9900007207";
    let latest_date_range = format!(
        "{}-{}-{}~{}-{}-{}",
        latest_date.year(),
        latest_date.month(),
        latest_date.day(),
        latest_date.year(),
        latest_date.month(),
        latest_date.day() + 7
    );
    let result = fetch(Query {
        url: "https://www.cninfo.com.cn/new/hisAnnouncement/query",
        params: QueryParams {
            stock: zhe_shuang_bank_code.to_string(),
            seDate: latest_date_range,
        },
    })
    .await?;
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
