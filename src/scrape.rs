use futures::StreamExt;
use indicatif::ProgressBar;
use log::{debug, info};
use reqwest::header::USER_AGENT;
use scraper::Html;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;

use super::epub_util;
use super::URL;

#[derive(Debug, Clone)]
pub struct Chapter {
    pub novel_name: String,
    pub url: String,
    pub name: String,
}

//TODO : implement local cache check
pub async fn scrape_novel(url: String) -> Result<(), reqwest::Error> {
    let res = reqwest::Client::new()
        .get(url)
        .header(
            USER_AGENT,
            "Mozilla/5.0 (X11; Fedora; Linux x86_64; rv:88.0) Gecko/20100101 Firefox/88.0",
        )
        .send()
        .await?;

    let body = res.text().await?;
    let html = Html::parse_document(body.as_str());
    let ch_selector = scraper::Selector::parse("ul.chapter-list > a.chapter-item").unwrap();

    let mut chapters: Vec<Chapter> = vec![];
    let sel = html.select(&ch_selector);
    let title_selector = scraper::Selector::parse("div.book-name").unwrap();

    //add scraped data to form Chapter
    sel.into_iter().for_each(|x| {
        chapters.push(Chapter {
            novel_name: html
                .select(&title_selector)
                .next()
                .unwrap()
                .text()
                .collect::<String>(),
            url: x.value().attr("href").unwrap().to_string(),
            name: x.text().collect::<String>(),
        });
    });

    //check if folder exists & create if it doesn't
    let name = html
        .select(&title_selector)
        .next()
        .unwrap()
        .text()
        .collect::<String>();

    println!("Book Name: {} \n", name);

    if !Path::new(format!("./{}", name).as_str()).exists() {
        info!("dir doesn't exist creating");
        std::fs::create_dir(Path::new(format!("./{}", name).as_str())).unwrap();
    }

    //write list of chapters in order -> will be useful in creating epub i guess ?
    //FIXME : After implementing local cahce check

    if Path::new(format!("./{}/info.html", name).as_str()).exists() {
        std::fs::remove_file(format!("./{}/info.html", name)).unwrap();
    } //we do this since we append stuff and if ran again can mess up stuff

    chapters.clone().into_iter().for_each(|x| {
        let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(format!("./{}/info.html", x.novel_name))
            .unwrap();

        if let Err(e) = writeln!(file, "<p>{}</p><br/>", x.name.as_str()) {
            eprintln!("Couldn't write to file: {}", e);
        }
    });

    //progress bar
    let pb = ProgressBar::new(chapters.len() as u64);
    let chap = chapters.clone();

    println!("Downloading Book : \n");
    let tasks = futures::stream::iter(chapters.into_iter().map(|chapter| async {
        pb.inc(1);
        save_html(chapter).await
    }))
    .buffer_unordered(5)
    .collect::<Vec<_>>();

    tasks.await;
    pb.finish();

    println!("Downloaded! Binding books into epub");

    //TODO: Create a epub with the chapterlist
    epub_util::create_epub(chap).unwrap();
    Ok(())
}

pub async fn save_html(chapter: Chapter) -> Result<(), reqwest::Error> {
    //TODO : remove illegal characters from foldername & filename

    debug!("{:#?}", chapter.url);

    let body = reqwest::Client::new()
        .get(format!("{}{}", URL, chapter.url))
        .header(
            USER_AGENT,
            "Mozilla/5.0 (X11; Arch; Linux x86_64; rv:88.0) Gecko/20100101 Firefox/88.0",
        )
        .send()
        .await?
        .text()
        .await?;

    let cht_selector = scraper::Selector::parse("div.chapter-entity").unwrap();
    let html = Html::parse_document(body.as_str());

    html.select(&cht_selector).into_iter().for_each(|x| {
        std::fs::write(
            format!("./{}/{}", chapter.novel_name, chapter.name),
            format!("<h1>{}</h1>{}",chapter.name,
                x.html()
            .replace("Find authorized novels in Webnovel，faster updates, better experience，Please click www.webnovel.com  for visiting.", "")),
        )
        .expect("Unable to write to file");
    });

    Ok(())
}
