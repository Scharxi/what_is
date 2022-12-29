use reqwest::Response;


async fn make_request(word: String) -> Result<Response, reqwest::Error> {
    let result = reqwest::get(format!("https://api.dictionaryapi.dev/api/v2/entries/en/{word}")).await?;

    Ok(result)
}


#[tokio::main]
async fn main() -> reqwest::Result<()> {
    match make_request("hello".to_owned()).await {
        Ok(def) => println!("{}", def.text().await?), 
        Err(e) => return Err(e)
    }; 

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
