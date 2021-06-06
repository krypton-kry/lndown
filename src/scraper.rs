use super::{epub_util, wuxiaworldco, URL};
use futures::StreamExt;
use indicatif::ProgressBar;
use log::{debug, info};
use reqwest::header::USER_AGENT;
use scraper::{Html, Selector};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct Novel {
    pub chapters: Vec<Chapter>,
    pub metadata: Metadata,
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
    //check url and choose module to use
    let url_base: Vec<&str> = url.split("/").collect();
    let selector: SelectorData = if url_base[2] == "www.wuxiaworld.co" {
        wuxiaworldco::get_data()
    } else {
        println!("{:#?}", url_base);
        panic!("Source Not Found!");
    };

    //scrape data
    let client = reqwest::Client::new();
    let body = &client
        .get(url)
        .header(
            USER_AGENT,
            "Mozilla/5.0 (X11; Fedora; Linux x86_64; rv:88.0) Gecko/20100101 Firefox/88.0",
        )
        .send()
        .await?
        .text()
        .await?;

    let html = Html::parse_document(body.as_str());
    let ch_selector = Selector::parse(&selector.index_selector).unwrap();
    let mut chapters: Vec<Chapter> = vec![];

    let title_selector = Selector::parse(selector.title_selector.as_str()).unwrap();
    let status_selector = Selector::parse(&selector.status_selector).unwrap();
    let author_selector = Selector::parse(&selector.author_selector).unwrap();

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

    //check and create folder to store html

    if !Path::new(format!("./{}", &metadata.title).as_str()).exists() {
        info!("dir doesn't exist creating");
        std::fs::create_dir(Path::new(format!("./{}", &metadata.title).as_str())).unwrap();
    }

    //save scraped data

    let pb = ProgressBar::new(chapters.len() as u64);
    let chap = chapters.clone();
    println!("{:#?}", metadata);

    println!("Downloading Book : \n");
    let tasks = futures::stream::iter(chapters.into_iter().map(|chapter| async {
        pb.inc(1);
        save_html(chapter, &client, &metadata, &selector).await
    }))
    .buffer_unordered(threads)
    .collect::<Vec<_>>();

    tasks.await;
    pb.finish();

    //use it to create epub

    println!("Downloaded! Binding books into epub");
    let novel: Novel = Novel {
        chapters: chap,
        metadata,
    };

    epub_util::create_epub(novel).unwrap();
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
