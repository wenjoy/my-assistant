use std::path::Path;

use tokio::fs;

pub fn read_pdf() {
    // TODO: use either pdf_extract or oxidize_pdf
    let bytes = std::fs::read("/Users/wenjoy/Downloads/demo.pdf").unwrap();
    let content = pdf_extract::extract_text_from_mem(&bytes).unwrap();
    println!("content is {content:?}, len: {}", bytes.len());
    // let mut reader = PdfReader::open("/Users/wenjoy/Downloads/demo.pdf").unwrap();
    // let document = reader.into_document();
    // let text = document.extract_text();

    // for (page_num, page_text) in text.iter().enumerate() {
    //     println!("Page {}: {:?}", page_num + 1, page_text);
    // }
}
async fn save_pdf(filename: String, content: bytes::Bytes) -> Result<(), std::io::Error> {
    let path = Path::new("static").join(&filename);

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).await?;
    }

    fs::write(path, content).await?;
    Ok(())
}
pub async fn fetch_pdf(filename: &str) -> Result<(), reqwest::Error> {
    let host = "https://static.cninfo.com.cn/";
    let url = String::from(host) + filename;
    let bytes = reqwest::get(&url).await?.bytes().await?;
    // println!("bytes, {bytes:?}");
    save_pdf(filename.to_string(), bytes).await.unwrap();
    Ok(())
}
