use reqwest::{Response};
use serde::Deserialize;

pub async fn make_request(word: String) -> std::result::Result<Response, reqwest::Error> {
    let res = reqwest::get(format!("https://api.dictionaryapi.dev/api/v2/entries/en/{word}")).await?;
    Ok(res)
}

#[async_trait::async_trait]
pub trait WordLookUp {
    async fn to_word_defenition(self) -> Vec<WordDefenition>;
}

#[async_trait::async_trait]
impl WordLookUp for Response {
    async fn to_word_defenition(self) ->  Vec<WordDefenition> {
        self.json::<Vec<WordDefenition>>().await.unwrap()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Definition {
    definition: String, 
    example: Option<String>, 
    synonyms: Vec<String>, 
    antonyms: Vec<String>
}

impl Definition {
    pub fn definition(&self) -> &str {
        self.definition.as_ref()
    }

    pub fn example(&self) -> Option<String> {
        self.example.clone()
    }

    pub fn synonyms(&self) -> Vec<String> {
        self.synonyms.clone()
    }

    pub fn antonyms(&self) -> Vec<String> {
        self.antonyms.clone()
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
    phonetic: Option<String>, 
    phonetics: Vec<Phonectic>, 
    origin: Option<String>, 
    meanings: Vec<Meaning>
}

impl WordDefenition {
    pub fn get_word(&self) -> String {
        self.word.clone().clone()
    }

    pub fn get_phonetic(&self) -> Option<String> {
        self.phonetic.clone()
    }

    pub fn get_phonetics(&self) -> Vec<Phonectic> {
        self.phonetics.clone()
    }

    pub fn get_origin(&self) -> Option<String> {
        self.origin.clone()
    }

    pub fn get_meanings(&self) -> Vec<Meaning> {
        self.meanings.clone()
    }

}