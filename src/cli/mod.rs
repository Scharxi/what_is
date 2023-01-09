use owo_colors::{OwoColorize, colors::{css::SeaGreen, Red}};

use crate::api::{self, WordLookUp};

pub mod parser;

pub async fn handle(query: String) {
    // make request to api
    let response = api::make_request(query.to_owned()).await;


    match response {
        Ok(res) => {
            if res.status().is_client_error() {
                eprintln!("{}: The word '{}' you're looking for does not exist.", "ERROR".red().bold(), query.green());
                return;
            }

            // convert response to obejct
            let word = res.to_word_definition().await;

            if word.is_err() {
                eprintln!("{}: Something went wrong while parsing response to object. \n\t {:?}", "ERROR".fg::<Red>().bold(), word.err().unwrap());
                return;
            }

            if let Some(word) = word.unwrap().first().cloned() {
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
        Err(e) => eprintln!("{}: Could not send request.\n\t {:?}", "ERROR".fg::<Red>().bold(), e)
    }
}
