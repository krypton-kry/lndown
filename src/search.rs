use crate::crawler::wuxiaworldco;
use dialoguer::console::Term;
use dialoguer::{theme::ColorfulTheme, Select};
use log::debug;
use std::process::exit;

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub name: String,
    pub url: String,
}

pub async fn search(query: &str) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let res: Vec<SearchResult> = wuxiaworldco::search(query.to_string(), &client).await?;

    //check if no results found
    if res.len() == 0 {
        println!("No results found");
        exit(1);
    }

    let mut names: Vec<String> = vec![];
    &res.clone().into_iter().for_each(|x| names.push(x.name));

    debug!("{:#?}", res);
    debug!("{:#?}", names);

    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&names)
        .default(0)
        .interact_on_opt(&Term::stderr())
        .unwrap()
        .unwrap();

    Ok(res[selection].url.clone())
}
