use reqwest::Response;
use serde::Deserialize;

pub async fn make_request(word: String) -> std::result::Result<Response, reqwest::Error> {
    let res = reqwest::get(format!("https://api.dictionaryapi.dev/api/v2/entries/en/{word}")).await?;
    Ok(res)
}

#[async_trait::async_trait]
pub trait WordLookUp {
    async fn to_word_defenition(self) ->  Result<Vec<WordDefenition>, reqwest::Error>;
}

#[async_trait::async_trait]
impl WordLookUp for Response {
    async fn to_word_defenition(self) -> Result<Vec<WordDefenition>, reqwest::Error>{
        self.json::<Vec<WordDefenition>>().await
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Definition {
    definition: String, 
    #[serde(skip_serializing)]
    example: Option<String>, 
    #[serde(skip_serializing)]
    synonyms: Vec<String>, 
    #[serde(skip_serializing)]
    antonyms: Vec<String>
}

impl Definition {
    pub fn definition(&self) -> &str {
        self.definition.as_ref()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Meaning {
    #[serde(rename="partOfSpeech")]
    part_of_speech: String, 
    definitions: Vec<Definition>
}

impl Meaning {
    pub fn get_part_of_speech(&self) -> String {
        self.part_of_speech.clone()
    }

    pub fn get_definitions(&self) -> Vec<Definition> {
        self.definitions.clone()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Phonectic {
    text: Option<String>, 
    audio: Option<String>
}

impl Phonectic {
    pub fn get_text(&self) -> Option<String> {
        self.text.clone()
    }

    pub fn get_audio(&self) -> Option<String> {
        self.audio.clone()
    }
}


#[derive(Debug, Clone, Deserialize)]
pub struct WordDefenition {
    word: String, 
    #[serde(skip_serializing)]
    phonetic: Option<String>, 
    #[serde(skip_serializing)]
    phonetics: Vec<Phonectic>,
    #[serde(skip_serializing)] 
    origin: Option<String>, 
    meanings: Vec<Meaning>
}

impl WordDefenition {
    pub fn get_word(&self) -> String {
        self.word.clone().clone()
    }

    pub fn get_meanings(&self) -> Vec<Meaning> {
        self.meanings.clone()
    }

}