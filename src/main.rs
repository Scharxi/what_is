
async fn make_request(word: String) -> Result<String, reqwest::Error> {
    let result = reqwest::get(format!("https://api.dictionaryapi.dev/api/v2/entries/en/{word}")).await?;

    Ok(result.text().await?)
}


#[tokio::main]
async fn main() -> reqwest::Result<()> {
    match make_request("hello".to_owned()).await {
        Ok(def) => println!("{}", def), 
        Err(e) => return Err(e)
    }; 

    Ok(())
}
