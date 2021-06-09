extern crate clap;
extern crate log;

use clap::{App, Arg};
pub mod epub_util;
mod search;
use log::info;
use search::search;

pub mod crawler;
use crate::crawler::wuxiaworldco;

const URL: &str = "https://www.wuxiaworld.co";
const LNDOWN: &str = "\n\u{2591}\u{2588}\u{2591}\u{2591}\u{2591}\u{2588}\u{2580}\u{2588}\u{2591}\u{2588}\u{2580}\u{2584}\u{2591}\u{2588}\u{2580}\u{2588}\u{2591}\u{2588}\u{2591}\u{2588}\u{2591}\u{2588}\u{2580}\u{2588}\n\u{2591}\u{2588}\u{2591}\u{2591}\u{2591}\u{2588}\u{2591}\u{2588}\u{2591}\u{2588}\u{2591}\u{2588}\u{2591}\u{2588}\u{2591}\u{2588}\u{2591}\u{2588}\u{2584}\u{2588}\u{2591}\u{2588}\u{2591}\u{2588}\n\u{2591}\u{2580}\u{2580}\u{2580}\u{2591}\u{2580}\u{2591}\u{2580}\u{2591}\u{2580}\u{2580}\u{2591}\u{2591}\u{2580}\u{2580}\u{2580}\u{2591}\u{2580}\u{2591}\u{2580}\u{2591}\u{2580}\u{2591}\u{2580}";

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    println!("{:}", LNDOWN);
    let matches = App::new("Light Novel Downloader")
        .version("0.3.1")
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
        .arg(
            Arg::with_name("threads")
                .short("t")
                .long("threads")
                .required(false)
                .help("No. of threads to be used (default : 5)\n(Please use with caution as site *will* block increased requests)")
                .takes_value(true),
        )
        .get_matches();

    if matches.is_present("query") && matches.is_present("url") {
        println!("Please use either --url or --query not both");
    }

    if !matches.is_present("query") && !matches.is_present("url") {
        println!("Please use lndown --help to view usage");
    }

    if matches.is_present("query") {
        let t = search(matches.value_of("query").unwrap()).await?;
        info!("Novel Url : {}{:#?}", URL, t);

        if matches.is_present("threads") {
            crawler::scrape_novel(
                format!("{}{}", URL, t),
                matches
                    .value_of("threads")
                    .unwrap()
                    .to_string()
                    .parse::<usize>()
                    .unwrap(),
            )
            .await?;
        } else {
            crawler::scrape_novel(format!("{}{}", URL, t), 5).await?;
        }
    }

    if matches.is_present("url") {
        info!("Novel Url : {:#?}", matches.value_of("url").unwrap());

        if matches.is_present("threads") {
            crawler::scrape_novel(
                matches.value_of("url").unwrap().to_string(),
                matches
                    .value_of("threads")
                    .unwrap()
                    .to_string()
                    .parse::<usize>()
                    .unwrap(),
            )
            .await?;
        } else {
            crawler::scrape_novel(matches.value_of("url").unwrap().to_string(), 5).await?;
        }
    }

    Ok(())
}

/*
TODO : add proxy
TODO : add number of chapters in selection screen
TODO : Check status code before adding to chapter list
TODO : non interactive query search & multiple downloads (download everything in search result)
TODO : use path given by user
TODO : add print details and exit (-d)
TODO : print more details after selecting book [search.rs]
TODO : add pagination to search
TODO : create only html option
TODO : add authenication
*/
