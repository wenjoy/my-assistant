use serde::Deserialize;
use std::{collections::HashMap, error::Error};
use time::OffsetDateTime;

pub mod db;
pub mod pdf;

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
    println!("status: {status}");

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
    let latest_date = OffsetDateTime::from_unix_timestamp(latest_date).unwrap();
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
