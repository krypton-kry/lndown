extern crate prettytable;

use std::process::exit;

use prettytable::*;
use reqwest::header::USER_AGENT;

use scraper::Html;

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

    //print out LightNovel names and author names
    let mut table = Table::new();
    table.add_row(row!["", Frbc->"LightNovel", Frbc->"Author"]);

    for i in (0..r.len()).step_by(3) {
        //yeah i know there's probably a better way to do this
        if i == 0 {
            table.add_row(row![Frbc->i + 1, Frbc->&r[i], Frbc->&r[i + 1]]);
        } else {
            table.add_row(row![Frbc->(i / 3) + 1, Frbc->&r[i], Frbc->&r[i + 1]]);
        }
    }

    table.printstd();
    println!("Select Novel To Download : ");

    //get user input
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Error: Unable to read user input");
    input.truncate(input.len() - 1);

    let i = (input.parse::<i32>().unwrap() * 3) - 1;
    let result = r.get(i as usize).unwrap();

    Ok(result.to_string())
}
