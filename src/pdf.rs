pub fn read_pdf() {}
pub async fn fetch_pdf(url: &str) -> Result<(), reqwest::Error> {
    let host = "https://static.cninfo.com.cn/";
    let url = String::from(host) + url;
    // let client = reqwest::Client::new();
    let pdf = reqwest::get(url).await?.text().await;
    println!("pdf, {pdf:?}");
    Ok(())
}
