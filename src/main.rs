use reqwest::{self};
use scraper::{Html, Selector};
use std::error::Error;

async fn fetch_news(url: &str) -> Result<(), Box<dyn Error>> {
let body = reqwest::get(url)
.await?
.text()
.await?;

let document = Html::parse_document(&body);

let selector = Selector::parse("h2").unwrap();

let mut titles = Vec::new();
for element in document.select(&selector) {
    let title = element.text().collect::<Vec<_>>().join(" ");
    titles.push(title);
}
if titles.is_empty() {
eprint!("Nenhuma notícia encontrada ");
}else {
    for (i, title) in titles.iter().enumerate(){
        println!("{} - {}", i + 1, title);
    }
}
Ok(())
}
#[tokio::main]
async fn main() {

    let url = "https://g1.globo.com/";

    if let Err(e) = fetch_news(url).await {
eprintln!("Erro ao buscar novas notícias, tente novamente! {}", e);        
    }
}