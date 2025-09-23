use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let url = "https://httpbin.org/ip";
    // let url = "https://httpbin.org/anything";
    // let url = "http://localhost:8080";
    let url = "https://www.cninfo.com.cn/new/hisAnnouncement/query";

    let client = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .build()?;

    let mut map = HashMap::new();
    map.insert("stock", "601916,9900007207");
    // map.insert("tabName", "fulltext");
    // map.insert("pageSize", "30");
    // map.insert("pageNumber", "1");
    // map.insert("plate", "sh");
    // map.insert("plate", "sh");
    // map.insert("isHLtitle", "true");

    // let forms:Vec<_> = map.into_iter().collect();
    let request = client.post(url).form(&map);

    // let resp = reqwest::get(url).await?.json::<HashMap<String, String>>().await?;
    let resp = request.send().await?;

    let status = resp.status();
    println!("status: {status}");

    // let result = resp.json::<HashMap<String, String>>().await?;
    let result = resp.text().await?;

    println!("{result:#?}");
    Ok(())
}
