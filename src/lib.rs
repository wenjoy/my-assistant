use serde::Deserialize;
use std::{collections::HashMap, error::Error};

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

pub async fn fetch() -> Result<Response, Box<dyn Error>> {
    let url = "https://www.cninfo.com.cn/new/hisAnnouncement/query";

    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .build()?;

    let mut map = HashMap::new();
    map.insert("stock", "601916,9900007207");

    let request = client.post(url).form(&map);

    let resp = request.send().await?;

    let status = resp.status();
    println!("status: {status}");

    let result: Response = resp.json().await?;

    println!("{result:#?}");
    Ok(result)
}
