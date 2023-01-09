#![allow(dead_code)]
mod api;
use api::*;
mod cli;
use cli::{*, parser::CLI};

#[tokio::main]
async fn main() -> reqwest::Result<()> {
    let input = CLI::get_input(); 
    handle(input).await;

    Ok(())
}

#[cfg(tests)]
mod tests; 

#[tokio::test]
async fn test_make_request() {
    let res = make_request("hello".to_owned()).await; 
    assert!(res.is_ok());
    assert_eq!(res.as_ref().unwrap().status().is_success(), true);
    let res = make_request("adgawdjmab".to_owned()).await; 
    assert!(res.unwrap().status().is_client_error());
}

#[tokio::test]
async fn test_turn_response_to_word_def() {
    let res = make_request("hello".to_owned()).await;
    let words = res.unwrap().to_word_definition().await;
    assert_eq!(words.unwrap().first().unwrap().get_word(), "hello".to_owned())
}
