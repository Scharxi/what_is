use clap::ArgMatches;
use owo_colors::{OwoColorize, colors::css::SeaGreen};

use crate::api::{self, WordLookUp};

pub mod parser;

pub async fn handle(matches: &ArgMatches) {
    let query = matches.get_one::<String>("word").unwrap();

    // make request to api
    let response = api::make_request(query.to_owned()).await;

    match response {
        Ok(res) => {
            // convert response to obejct
            let word = res.to_word_defenition().await;

            if let Some(word) = word.first().cloned() {
                let meanings = word.get_meanings();
                let definition: String = meanings
                    .iter()
                    .map(|m| {
                        m.get_definitions()
                            .iter()
                            .map(|d| d.definition().to_owned())
                            .collect::<Vec<String>>()
                            .join("\n\n")
                    })
                    .collect::<String>();
                println!("{} {}\n", "Here are the definitions for".fg::<SeaGreen>().underline(), query.underline());
                println!("{}", definition);
            }
        }
        Err(e) => {}
    }
}
