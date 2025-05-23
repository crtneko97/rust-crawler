use reqwest::Error;
use scraper::{Html, Selector};


#[tokio::main]
async fn main() -> Result<(), Error> 
{
    let url = "https://github.com/crtneko97/crawler-java";
    let body = reqwest::get(url).await?
        .text().await?;

    let document = Html::parse_document(&body);
    let selector = Selector::parse("a").unwrap();

    println!("Links found at: {}", url);
    for elem in document.select(&selector)
    {
        if let Some(link) = elem.value().attr("href")
        {
            println!("- {}", link);
        }
    }
    Ok(())
}
