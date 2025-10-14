use serde::{Deserialize, Deserializer};
use sqlx::Row;
use sqlx::{Connection, SqliteConnection};
use std::{collections::HashMap, error::Error};
use time::OffsetDateTime;

use crate::db::{initial_database, insert_data, query_data, query_latest_data};
use crate::pdf::fetch_pdf;

pub mod db;
pub mod pdf;

fn deserialize_nullable_vec<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    let opt = Option::<Vec<T>>::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}

#[derive(Deserialize, Debug)]
pub struct Announcement {
    announcementId: String,
    announcementTitle: String,
    announcementTime: i64,
    adjunctType: String,
    adjunctUrl: String,
}
#[derive(Deserialize, Debug)]
pub struct Response {
    #[serde(default, deserialize_with = "deserialize_nullable_vec")]
    pub announcements: Vec<Announcement>,
}

pub struct Query {
    pub url: &'static str,
    pub params: QueryParams,
}

pub struct QueryParams {
    pub stock: String,
    pub seDate: Option<String>,
}

pub async fn fetch(Query { url, params }: Query) -> Result<Response, Box<dyn Error>> {
    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .build()?;

    let mut map = HashMap::new();
    map.insert("stock", params.stock.as_str());

    let se_date = params.seDate.unwrap_or("".to_string());
    map.insert("seDate", se_date.as_str());

    let request = client.post(url).form(&map);

    let resp = request.send().await?;

    let status = resp.status();
    println!("fetch status: {status}");

    let result: Response = resp.json().await?;
    println!("fetch result: {result:#?}");
    Ok(result)
}

pub async fn fetch_all() -> Result<Response, Box<dyn Error>> {
    let zhe_shuang_bank_code = "601916,9900007207";
    let result = fetch(Query {
        url: "https://www.cninfo.com.cn/new/hisAnnouncement/query",
        params: QueryParams {
            stock: zhe_shuang_bank_code.to_string(),
            seDate: None,
        },
    })
    .await?;

    Ok(result)
}

pub async fn fetch_latest_data(
    latest_date: i64,
    range: Option<u32>,
) -> Result<Response, Box<dyn Error>> {
    let latest_date = OffsetDateTime::from_unix_timestamp(latest_date / 1000).unwrap();
    let zhe_shuang_bank_code = "601916,9900007207";
    let range = range.unwrap_or(7);
    let latest_date_range = format!(
        "{}-{}-{}~{}-{}-{}",
        latest_date.year(),
        latest_date.month(),
        latest_date.day(),
        latest_date.year(),
        latest_date.month(),
        u32::from(latest_date.day()) + range
    );
    println!("latest date range {}", latest_date_range);
    let result = fetch(Query {
        url: "https://www.cninfo.com.cn/new/hisAnnouncement/query",
        params: QueryParams {
            stock: zhe_shuang_bank_code.to_string(),
            seDate: Some(latest_date_range),
        },
    })
    .await?;
    Ok(result)
}

pub async fn crawl() -> Result<(), Box<dyn Error>> {
    let mut conn = SqliteConnection::connect("sqlite:data.db").await?;
    initial_database(&mut conn).await?;
    //TODO: extract this into saparate bin
    // clear_database(&mut conn).await?;

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

    let res = query_data(&mut conn).await?;
    for item in &res {
        let pdf_url = item.try_get::<String, _>("adjunct_url").unwrap();
        println!("{}", pdf_url);
        fetch_pdf(&pdf_url).await?;
    }
    Ok(())
}
