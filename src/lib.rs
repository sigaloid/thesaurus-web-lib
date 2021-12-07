#![deny(
    anonymous_parameters,
    clippy::all,
    const_err,
    illegal_floating_point_literal_pattern,
    late_bound_lifetime_arguments,
    path_statements,
    patterns_in_fns_without_body,
    rust_2018_idioms,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_extern_crates
)]
#![warn(
    clippy::dbg_macro,
    clippy::decimal_literal_representation,
    clippy::get_unwrap,
    // clippy::missing_docs_in_private_items,
    clippy::nursery,
    clippy::pedantic,
    clippy::todo,
    clippy::unimplemented,
    clippy::unwrap_used,
    clippy::use_debug,
    missing_copy_implementations,
    clippy::all,
    // missing_debug_implementations,
    unused_qualifications,
    variant_size_differences
)]
#![allow(dead_code, clippy::too_many_lines)]

pub fn lookup_word(word: &String) -> Option<SynonymInfo> {
    let request = ureq::get(&format!(
        "https://tuna.thesaurus.com/pageData/{}",
        urlencoding::encode(&word)
    ))
    .call()
    .ok()?
    .into_string()
    .ok()?;
    let object: SynonymInfo = nanoserde::DeJson::deserialize_json(&request).ok()?;
    Some(object)
}


// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::[object Object];
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: [object Object] = serde_json::from_str(&json).unwrap();
// }

// extern crate serde_derive;
use nanoserde::{DeJson, SerJson};
use serde::{Deserialize, Serialize};
#[derive(SerJson, DeJson, Deserialize, Serialize, Clone)]
pub struct SynonymInfo {
    #[nserde(rename = "data")]
    pub data: Data,
}

#[derive(SerJson, DeJson, Deserialize, Serialize, Clone)]
pub struct Data {
    #[nserde(rename = "definitionData")]
    pub definition_data: DefinitionData,

    // #[nserde(rename = "categoryId")]
    // category_id: String,
    #[nserde(rename = "pronunciation")]
    pub pronunciation: Option<Pronunciation>,

    // #[nserde(rename = "confusables")]
    // confusables: Vec<Option<serde_json::Value>>,
    #[nserde(rename = "supplementaryNotes")]
    pub supplementary_notes: Option<Vec<SupplementaryNote>>,

    // #[nserde(rename = "etymology")]
    // etymology: Vec<Option<serde_json::Value>>,
    #[nserde(rename = "exampleSentences")]
    pub example_sentences: Option<Vec<ExampleSentence>>,
    // #[nserde(rename = "slugLuna")]
    // slug_luna: String,
}
#[derive(SerJson, DeJson, Deserialize, Serialize, Clone)]
pub struct SupplementaryNote {
    #[nserde(rename = "type")]
    pub supplementary_note_type: String,

    #[nserde(rename = "content")]
    pub content: String,
}

#[derive(SerJson, DeJson, Deserialize, Serialize, Clone)]
pub struct DefinitionData {
    #[nserde(rename = "entry")]
    pub entry: String,

    #[nserde(rename = "type")]
    definition_data_type: String,

    #[nserde(rename = "definitions")]
    pub definitions: Vec<Definition>,

    #[nserde(rename = "slug")]
    slug: String,

    #[nserde(rename = "rawSlug")]
    raw_slug: String,

    #[nserde(rename = "searchQueries")]
    search_queries: Vec<String>,
}

#[derive(SerJson, DeJson, Deserialize, Serialize, Clone)]
pub struct Definition {
    // #[nserde(rename = "isInformal")]
    // is_informal: Option<serde_json::Value>,
    #[nserde(rename = "isVulgar")]
    is_vulgar: String,

    #[nserde(rename = "definition")]
    pub definition: String,

    // #[nserde(rename = "thesRid")]
    // thes_rid: String,
    #[nserde(rename = "pos")]
    pos: String,

    #[nserde(rename = "synonyms")]
    pub synonyms: Vec<Onym>,

    #[nserde(rename = "antonyms")]
    pub antonyms: Vec<Onym>,
    // #[nserde(rename = "note")]
    // note: Option<serde_json::Value>,
}

#[derive(SerJson, DeJson, Deserialize, Serialize, Clone)]
pub struct Onym {
    #[nserde(rename = "similarity")]
    pub similarity: String,

    #[nserde(rename = "isInformal")]
    is_informal: String,

    // #[nserde(rename = "isVulgar")]
    // is_vulgar: Option<serde_json::Value>,
    #[nserde(rename = "term")]
    pub term: String,

    #[nserde(rename = "targetTerm")]
    pub target_term: Option<String>,

    #[nserde(rename = "targetSlug")]
    pub target_slug: Option<String>,
}

#[derive(SerJson, DeJson, Deserialize, Serialize, Clone)]
pub struct ExampleSentence {
    #[nserde(rename = "source")]
    source: Source,

    #[nserde(rename = "profanity")]
    profanity: i64,

    #[nserde(rename = "sentence")]
    pub sentence: String,
}

#[derive(SerJson, DeJson, Deserialize, Serialize, Clone)]
pub struct Source {
    #[nserde(rename = "source_name")]
    source_source_name: Option<String>,

    #[nserde(rename = "author")]
    author: String,

    #[nserde(rename = "url")]
    url: String,

    #[nserde(rename = "title")]
    title: String,

    #[nserde(rename = "sourceName")]
    source_name: Option<String>,

    #[nserde(rename = "publicationDate")]
    publication_date: Option<String>,

    #[nserde(rename = "publication_date")]
    source_publication_date: Option<String>,

    #[nserde(rename = "type")]
    source_type: Type,

    #[nserde(rename = "abbreviation")]
    abbreviation: Abbreviation,
}

#[derive(SerJson, DeJson, Deserialize, Serialize, Clone)]
pub struct Pronunciation {
    #[nserde(rename = "ipa")]
    pub ipa: Option<String>,

    #[nserde(rename = "spell")]
    pub spell: Option<String>,

    #[nserde(rename = "audio")]
    pub audio: Option<Audio>,
}

#[derive(SerJson, DeJson, Deserialize, Serialize, Clone)]
pub struct Audio {
    #[nserde(rename = "audio/ogg")]
    pub audio_ogg: String,

    #[nserde(rename = "audio/mpeg")]
    pub audio_mpeg: String,
}

#[derive(SerJson, DeJson, Deserialize, Serialize, Clone, Copy)]
pub enum Abbreviation {
    #[nserde(rename = "gbg")]
    Gbg,

    #[nserde(rename = "nlp")]
    Nlp,
}

#[derive(SerJson, DeJson, Deserialize, Serialize, Clone, Copy)]
pub enum Type {
    #[nserde(rename = "const")]
    Const,
}



pub fn suggestions(frag: String, limit: usize) -> Option<Vec<(String, String)>> {
    let request = ureq::get(&format!(
        "https://api-portal.dictionary.com/dcom/list/{}?limit={}",
        urlencoding::encode(&frag), limit
    ))
    .call()
    .ok()?
    .into_string()
    .ok()?;
    let object: Root = nanoserde::DeJson::deserialize_json(&request).ok()?;
    let mut v = vec![];
    for d in object.data { 
        v.push((d.display_form, d.url))
    }
    Some(v)
}


#[derive(SerJson, DeJson)]
pub struct Root {
    #[nserde(rename = "data")]
    data: Vec<Datum>,

    #[nserde(rename = "meta")]
    meta: Meta,
}

#[derive(SerJson, DeJson)]
pub struct Datum {
    #[nserde(rename = "url")]
    url: String,

    #[nserde(rename = "displayForm")]
    display_form: String,

    #[nserde(rename = "nearbyWordOrdinal")]
    nearby_word_ordinal: Option<i64>,

    #[nserde(rename = "rank")]
    rank: Option<i64>,

    #[nserde(rename = "tunaSlug")]
    tuna_slug: Option<String>,
}

#[derive(SerJson, DeJson)]
pub struct Meta {
    #[nserde(rename = "totalResults")]
    total_results: i64,
}

