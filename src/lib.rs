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

pub fn lookup_word(word: impl ToString) -> Option<SynonymInfo> {
    let request = ureq::get(&format!(
        "https://tuna.thesaurus.com/pageData/{}",
        urlencoding::encode(&word.to_string())
    ))
    .call()
    .ok()?
    .into_string()
    .ok()?;

    let object: SynonymInfo = serde_json::from_str(&request).ok()?;
    Some(object)
}

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Clone)]
pub struct SynonymInfo {
    #[serde(rename = "data")]
    pub data: Data,
}
impl SynonymInfo {
    pub fn words(&self) -> Vec<String> {
        let mut x = vec![];
        for def in &self.data.definition_data.definitions {
            for i in &def.synonyms {
                x.push(i.term.clone());
            }
            for i in &def.antonyms {
                x.push(i.term.clone());
            }
        }
        x
    }
    // pub fn synonyms(&self) -> Vec<(String, Vec<String>)> {
    //     let mut x = vec![];
    //     for def in &self.data.definition_data.definitions {
    //         let mut y = vec![];
    //         for i in &def.synonyms {
    //             y.push(i.term.clone());
    //         }
    //         x.push((&def.definition, y))
    //     }
    //     x
    // }
    pub fn defs(
        &self,
    ) -> (
        Option<Pronunciation>,
        Vec<(String, Vec<(String, String)>, Vec<(String, String)>)>,
    ) {
        let mut x = vec![];
        for def in &self.data.definition_data.definitions {
            let mut syns = vec![];
            for i in &def.synonyms {
                syns.push((i.term.clone(), i.similarity.clone()));
            }
            let mut ants = vec![];
            for i in &def.antonyms {
                ants.push((i.term.clone(), i.similarity.clone()));
            }

            x.push((def.definition.clone(), syns, ants))
        }
        (self.data.pronunciation.clone(), x)
    }
    // pub fn antonyms(&self) -> Vec<(String, Vec<String>)> {
    //     let mut x = vec![];
    //     for def in &self.data.definition_data.definitions {
    //         let mut y = vec![];
    //         for i in &def.antonyms {
    //             y.push(i.term.clone());
    //         }
    //         x.push((&def.definition, y))
    //     }
    //     x
    // }
}
#[derive(Serialize, Deserialize, Clone)]
pub struct Data {
    #[serde(rename = "definitionData")]
    pub definition_data: DefinitionData,

    // #[serde(rename = "categoryId")]
    // category_id: String,
    #[serde(rename = "pronunciation")]
    pub pronunciation: Option<Pronunciation>,
    // // #[serde(rename = "confusables")]
    // // confusables: Vec<Option<serde_json::Value>>,
    // #[serde(rename = "supplementaryNotes")]
    // pub supplementary_notes: Option<Vec<SupplementaryNote>>,
    //
    // // #[serde(rename = "etymology")]
    // // etymology: Vec<Option<serde_json::Value>>,
    #[serde(rename = "exampleSentences")]
    pub example_sentences: Option<Vec<ExampleSentence>>,
    // // #[serde(rename = "slugLuna")]
    // // slug_luna: String,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct SupplementaryNote {
    #[serde(rename = "type")]
    pub supplementary_note_type: String,

    #[serde(rename = "content")]
    pub content: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DefinitionData {
    #[serde(rename = "entry")]
    pub entry: String,

    #[serde(rename = "type")]
    definition_data_type: String,

    #[serde(rename = "definitions")]
    pub definitions: Vec<Definition>,

    #[serde(rename = "slug")]
    slug: String,

    #[serde(rename = "rawSlug")]
    raw_slug: String,

    #[serde(rename = "searchQueries")]
    search_queries: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Definition {
    // #[serde(rename = "isInformal")]
    // is_informal: Option<serde_json::Value>,
    #[serde(rename = "isVulgar")]
    is_vulgar: String,

    #[serde(rename = "definition")]
    pub definition: String,

    // #[serde(rename = "thesRid")]
    // thes_rid: String,
    #[serde(rename = "pos")]
    pub pos: String,

    #[serde(rename = "synonyms")]
    pub synonyms: Vec<Onym>,

    #[serde(rename = "antonyms")]
    pub antonyms: Vec<Onym>,
    // #[serde(rename = "note")]
    // note: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Onym {
    #[serde(rename = "similarity")]
    pub similarity: String,

    #[serde(rename = "isInformal")]
    is_informal: String,

    // #[serde(rename = "isVulgar")]
    // is_vulgar: Option<serde_json::Value>,
    #[serde(rename = "term")]
    pub term: String,

    #[serde(rename = "targetTerm")]
    pub target_term: Option<String>,

    #[serde(rename = "targetSlug")]
    pub target_slug: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ExampleSentence {
    #[serde(rename = "source")]
    pub source: Source,

    #[serde(rename = "profanity")]
    profanity: i64,

    #[serde(rename = "sentence")]
    pub sentence: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Source {
    #[serde(rename = "source_name")]
    source_source_name: Option<String>,

    #[serde(rename = "author")]
    pub author: String,

    #[serde(rename = "url")]
    pub url: String,

    #[serde(rename = "title")]
    pub title: String,

    #[serde(rename = "sourceName")]
    pub source_name: Option<String>,

    #[serde(rename = "publicationDate")]
    publication_date: Option<String>,

    #[serde(rename = "publication_date")]
    source_publication_date: Option<String>,

    #[serde(rename = "type")]
    source_type: Type,

    #[serde(rename = "abbreviation")]
    abbreviation: Abbreviation,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Pronunciation {
    #[serde(rename = "ipa")]
    pub ipa: Option<String>,

    #[serde(rename = "spell")]
    pub spell: Option<String>,

    #[serde(rename = "audio")]
    pub audio: Option<Audio>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Audio {
    #[serde(rename = "audio/ogg")]
    pub audio_ogg: String,

    #[serde(rename = "audio/mpeg")]
    pub audio_mpeg: String,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum Abbreviation {
    #[serde(rename = "gbg")]
    Gbg,

    #[serde(rename = "nlp")]
    Nlp,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum Type {
    #[serde(rename = "const")]
    Const,
}

// pub fn suggestions(frag: String, limit: usize) -> Option<Vec<(String, String)>> {
//     let request = ureq::get(&format!(
//         "https://api-portal.dictionary.com/dcom/list/{}?limit={}",
//         urlencoding::encode(&frag),
//         limit
//     ))
//     .call()
//     .ok()?
//     .into_string()
//     .ok()?;
//     let object: Root = nanoserde::DeJson::deserialize_json(&request).ok()?;
//     let mut v = vec![];
//     for d in object.data {
//         v.push((d.display_form, d.url))
//     }
//     Some(v)
// }

#[derive(Serialize, Deserialize)]
pub struct Root {
    #[serde(rename = "data")]
    data: Vec<Datum>,

    #[serde(rename = "meta")]
    meta: Meta,
}

#[derive(Serialize, Deserialize)]
pub struct Datum {
    #[serde(rename = "url")]
    url: String,

    #[serde(rename = "displayForm")]
    display_form: String,

    #[serde(rename = "nearbyWordOrdinal")]
    nearby_word_ordinal: Option<i64>,

    #[serde(rename = "rank")]
    rank: Option<i64>,

    #[serde(rename = "tunaSlug")]
    tuna_slug: Option<String>,
}

#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct Meta {
    #[serde(rename = "totalResults")]
    total_results: i64,
}
