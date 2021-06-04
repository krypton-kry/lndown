extern crate clap;
extern crate log;

use clap::{App, Arg};
pub mod epub_util;
mod search;
use log::info;
use search::search;
pub mod scrape;

const URL: &str = "https://www.wuxiaworld.co";
const LNDOWN: &str = "\n\u{2591}\u{2588}\u{2591}\u{2591}\u{2591}\u{2588}\u{2580}\u{2588}\u{2591}\u{2588}\u{2580}\u{2584}\u{2591}\u{2588}\u{2580}\u{2588}\u{2591}\u{2588}\u{2591}\u{2588}\u{2591}\u{2588}\u{2580}\u{2588}\n\u{2591}\u{2588}\u{2591}\u{2591}\u{2591}\u{2588}\u{2591}\u{2588}\u{2591}\u{2588}\u{2591}\u{2588}\u{2591}\u{2588}\u{2591}\u{2588}\u{2591}\u{2588}\u{2584}\u{2588}\u{2591}\u{2588}\u{2591}\u{2588}\n\u{2591}\u{2580}\u{2580}\u{2580}\u{2591}\u{2580}\u{2591}\u{2580}\u{2591}\u{2580}\u{2580}\u{2591}\u{2591}\u{2580}\u{2580}\u{2580}\u{2591}\u{2580}\u{2591}\u{2580}\u{2591}\u{2580}\u{2591}\u{2580}";

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    println!("{:}", LNDOWN);
    let matches = App::new("Light Novel Downloader")
        .version("0.1")
        .arg(
            Arg::with_name("query")
                .short("q")
                .long("query")
                .required(false)
                .help("Query to search")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("url")
                .short("u")
                .long("url")
                .required(false)
                .help("Url of Lightnovel to Download")
                .takes_value(true),
        )
        .get_matches();

    if matches.is_present("query") && matches.is_present("url") {
        println!("Please use either --url or --query not both");
    }

    if !matches.is_present("query") && !matches.is_present("url") {
        println!("Please use either --url or --query");
    }

    if matches.is_present("query") {
        let t = search(matches.value_of("query").unwrap()).await?;
        info!("Novel Url : {}{:#?}", URL, t);

        scrape::scrape_novel(format!("{}{}", URL, t)).await?;
    }

    if matches.is_present("url") {
        info!("Novel Url : {:#?}", matches.value_of("url").unwrap());
        scrape::scrape_novel(matches.value_of("url").unwrap().to_string()).await?;
    }

    Ok(())
}
/*
TODO : non interactive query search
TODO : use path given by user
TODO : add a framwork so as to create modules for different sites
*/
