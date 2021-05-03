use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
pub struct Jisho {
    pub meta: JishoMeta,
    pub data: Option<Vec<JishoDataItem>>,
}

#[derive(Deserialize, Debug)]
pub struct JishoMeta {
    pub status: u32,
}

#[derive(Deserialize, Debug)]
pub struct JishoDataItem {
    pub slug: String,
    pub is_common: Option<bool>,
    pub tags: Vec<String>,
    pub jlpt: Vec<String>,
    pub japanese: Vec<JishoDataJapanese>,
    pub senses: Vec<JishoDataSenses>,
    pub attribution: JishoDataAttribution,
}

#[derive(Deserialize, Debug)]
pub struct JishoDataJapanese {
    pub word: Option<String>,
    pub reading: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct JishoDataSenses {
    pub english_definitions: Vec<String>,
    pub parts_of_speech: Vec<String>,
    pub links: Vec<JishoDataLinks>,
    pub tags: Vec<String>,
    pub restrictions: Vec<String>,
    pub see_also: Vec<String>,
    pub antonyms: Vec<String>,
    pub source: Vec<JishoDataSource>,
    pub info: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct JishoDataAttribution {
    pub jmdict: Value,
    pub jmnedict: Value,
    pub dbpedia: Value,
}

#[derive(Deserialize, Debug)]
pub struct JishoDataLinks {
    pub text: String,
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct JishoDataSource {
    pub language: String,
    pub word: String,
}
