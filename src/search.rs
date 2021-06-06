use dialoguer::console::Term;
use dialoguer::{theme::ColorfulTheme, Select};
use log::debug;
use reqwest::header::USER_AGENT;
use scraper::Html;
use std::convert::TryFrom;
use std::process::exit;

pub async fn search(query: &str) -> Result<String, reqwest::Error> {
    let res = reqwest::Client::new()
        .get(format!(
            "{}{}/1",
            "https://www.wuxiaworld.co/search/", query
        ))
        .header(
            USER_AGENT,
            "Mozilla/5.0 (X11; Fedora; Linux x86_64; rv:88.0) Gecko/20100101 Firefox/88.0",
        )
        .send()
        .await?;

    let body = res.text().await?;
    let html = Html::parse_document(body.as_str());
    let search_selector =
        scraper::Selector::parse("ul.result-list > li > div.item-info > a.book-name").unwrap();

    let mut r: Vec<_> = vec![];
    let sel = html.select(&search_selector);

    //order 0 : LN Name, 1 : Author, 2 : href
    sel.into_iter().for_each(|x| {
        r.append(&mut x.text().collect::<Vec<_>>());
        r.append(&mut vec![x.value().attr("href").unwrap()]);
    });

    //check if no results found
    if r.len() == 0 {
        println!("No results found");
        exit(1);
    }

    let mut names: Vec<String> = vec![];

    &r.clone()
        .into_iter()
        .step_by(3)
        .for_each(|x| names.push(x.to_string()));

    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&names)
        .default(0)
        .interact_on_opt(&Term::stderr())
        .unwrap();

    match selection {
        Some(index) => {
            //i'm disappointing dam*it
            if index == 0 {
                Ok(r[2].to_string())
            } else {
                let res = ((i32::try_from(index).unwrap() * 3) + 2) as usize;
                debug!("link chosen :{}", r[res]);

                Ok(r[res].to_string())
            }
        }
        None => todo!(),
    }
}
