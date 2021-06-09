use crate::{
    crawler::{Chapter, Metadata, Novel, SelectorData},
    search::SearchResult,
};
use reqwest::header::USER_AGENT;
use scraper::{Html, Selector};

pub const BASE_URL: &str = "www.wuxiaworld.co";
pub const SEARCH_URL: &str = "https://www.wuxiaworld.co/search/";

fn get_data() -> SelectorData {
    SelectorData {
        index_selector: "ul.chapter-list > a.chapter-item".to_string(),
        title_selector: "div.book-name".to_string(),
        status_selector: "div.book-state".to_string(),
        author_selector: "div.author".to_string(),
        text_selector: "div.chapter-entity".to_string(),
    }
}

pub async fn get_novel(url: String, client: &reqwest::Client) -> Result<Novel, reqwest::Error> {
    let body = client
        .get(url)
        .header(
            USER_AGENT,
            "Mozilla/5.0 (X11; Fedora; Linux x86_64; rv:88.0) Gecko/20100101 Firefox/88.0",
        )
        .send()
        .await?
        .text()
        .await?;

    let data: SelectorData = get_data();
    let html = Html::parse_document(body.as_str());
    let ch_selector = Selector::parse(&data.index_selector).unwrap();
    let mut chapters: Vec<Chapter> = vec![];

    let title_selector = Selector::parse(&data.title_selector.as_str()).unwrap();
    let status_selector = Selector::parse(&data.status_selector).unwrap();
    let author_selector = Selector::parse(&data.author_selector).unwrap();

    //add scraped data to form Chapter
    html.select(&ch_selector).into_iter().for_each(|x| {
        chapters.push(Chapter {
            url: x.value().attr("href").unwrap().to_string(),
            name: x.text().collect::<String>(),
        });
    });

    //scrape metadata
    let metadata: Metadata = Metadata {
        title: html
            .select(&title_selector)
            .next()
            .unwrap()
            .text()
            .collect::<String>(),
        author: Some(
            html.select(&author_selector)
                .next()
                .unwrap()
                .text()
                .collect::<String>(),
        ),
        total_chapters: chapters.len(),
        status: Some(
            html.select(&status_selector)
                .next()
                .unwrap()
                .text()
                .collect::<String>(),
        ),
    };

    Ok(Novel {
        chapters,
        metadata,
        selector: data,
    })
}

pub async fn search(
    query: String,
    client: &reqwest::Client,
) -> Result<Vec<SearchResult>, reqwest::Error> {
    let mut result: Vec<SearchResult> = vec![];

    let body = client
        .get(format!("{}{}/1", SEARCH_URL, query))
        .header(
            USER_AGENT,
            "Mozilla/5.0 (X11; Fedora; Linux x86_64; rv:88.0) Gecko/20100101 Firefox/88.0",
        )
        .send()
        .await?
        .text()
        .await?;

    let html = Html::parse_document(body.as_str());
    let search_selector =
        scraper::Selector::parse("ul.result-list > li > div.item-info > a.book-name").unwrap();

    let sel = html.select(&search_selector);

    sel.into_iter().for_each(|x| {
        let s_res = SearchResult {
            name: x.text().collect::<Vec<_>>()[0].to_string(),
            url: x.value().attr("href").unwrap().to_string(),
        };
        result.push(s_res);
    });

    Ok(result)
}
