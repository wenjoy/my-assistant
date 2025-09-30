use my_assitant::{
    Announcement, Query, QueryParams,
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
    let res = query_data(&mut conn).await?;
    for item in res {
        println!(
            "url is {}",
            item.try_get::<i64, _>("announcement_time").unwrap()
        );
    }
    let zhe_shuang_bank_code = "601916,9900007207";
    let latest_date_range = "2025-09-01~2025-09-30";
    let result = fetch(Query {
        url: "https://www.cninfo.com.cn/new/hisAnnouncement/query",
        params: QueryParams {
            stock: zhe_shuang_bank_code,
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
