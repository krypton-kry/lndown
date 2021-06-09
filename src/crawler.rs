use super::{epub_util, URL};
use futures::StreamExt;
use indicatif::ProgressBar;
use log::{debug, info};
use reqwest::header::USER_AGENT;
use scraper::Html;
use std::path::Path;

#[path = "sources/wuxiaworldco.rs"]
pub mod wuxiaworldco;

#[derive(Debug, Clone)]
pub struct Novel {
    pub chapters: Vec<Chapter>,
    pub metadata: Metadata,
    pub selector: SelectorData,
}

#[derive(Debug, Clone)]
pub struct Metadata {
    pub title: String,
    pub author: Option<String>,
    pub total_chapters: usize,
    pub status: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Chapter {
    pub name: String,
    pub url: String,
}

//pretty useless but makes it look kinda lit so i'll let it be
#[derive(Debug, Clone)]
pub struct SelectorData {
    pub index_selector: String, // chapters from index
    pub title_selector: String,
    pub status_selector: String,
    pub author_selector: String,
    pub text_selector: String, // text without the title
}
//TODO : some sites probably don't have all chapters in index so -> pagination

pub async fn scrape_novel(url: String, threads: usize) -> Result<(), reqwest::Error> {
    //check url and choose module to use & scrape data

    //too stupid refactor please
    let url_base: Vec<&str> = url.split("/").collect();
    let client = reqwest::Client::new();

    let novel: Novel = if url_base[2] == wuxiaworldco::BASE_URL {
        wuxiaworldco::get_novel(url, &client).await?
    } else {
        panic!("Source Not found");
    };

    //check and create folder to store html

    if !Path::new(format!("./{}", &novel.metadata.title).as_str()).exists() {
        info!("dir doesn't exist creating");
        std::fs::create_dir(Path::new(format!("./{}", &novel.metadata.title).as_str())).unwrap();
    }

    //save scraped data

    let pb = ProgressBar::new(novel.chapters.len() as u64);
    let chap = novel.clone();
    println!("{:#?}", &novel.metadata);

    println!("Downloading Book : \n");

    let meta = novel.clone().metadata;
    let select = novel.clone().selector;

    let tasks = futures::stream::iter(novel.chapters.into_iter().map(|chapter| async {
        pb.inc(1);
        save_html(chapter, &client, &meta, &select).await
    }))
    .buffer_unordered(threads)
    .collect::<Vec<_>>();

    tasks.await;
    pb.finish();

    //use it to create epub

    println!("Downloaded! Binding books into epub");

    epub_util::create_epub(chap).unwrap();
    println!("Epub created!");

    Ok(())
}

pub async fn save_html(
    chapter: Chapter,
    client: &reqwest::Client,
    meta: &Metadata,
    sd: &SelectorData,
) -> Result<(), reqwest::Error> {
    //TODO : remove illegal characters from foldername & filename

    debug!("{:#?}", chapter.url);

    let body = client
        .get(format!("{}{}", URL, chapter.url))
        .header(
            USER_AGENT,
            "Mozilla/5.0 (X11; Fedora; Linux x86_64; rv:88.0) Gecko/20100101 Firefox/88.0",
        )
        .send()
        .await?
        .text()
        .await?;

    let cht_selector = scraper::Selector::parse(sd.text_selector.as_str()).unwrap();
    let html = Html::parse_document(body.as_str());

    html.select(&cht_selector).into_iter().for_each(|x| {
        std::fs::write(
            format!("./{}/{}", meta.title, chapter.name),
            format!("<h1>{}</h1>{}",chapter.name,
                x.html()
            .replace("Find authorized novels in Webnovel，faster updates, better experience，Please click www.webnovel.com  for visiting.", "")),
        )
        .expect("Unable to write to file");
    });

    Ok(())
}
