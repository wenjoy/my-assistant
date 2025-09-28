use oxidize_pdf::PdfReader;

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
pub async fn fetch_pdf(url: &str) -> Result<(), reqwest::Error> {
    let host = "https://static.cninfo.com.cn/";
    let url = String::from(host) + url;
    // let client = reqwest::Client::new();
    let pdf = reqwest::get(url).await?.text().await;
    println!("pdf, {pdf:?}");
    Ok(())
}
