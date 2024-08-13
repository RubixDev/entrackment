use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use struct_field_names_as_array::FieldNamesAsSlice;

use crate::{Key, OlId, OpenLibRequest};

#[derive(Debug, Clone, Copy, Default, strum::Display)]
#[non_exhaustive]
pub enum SearchKind {
    #[default]
    #[strum(serialize = "")]
    Books,
    #[strum(serialize = "/authors")]
    Authors,
    #[strum(serialize = "/subjects")]
    Subjects,
    #[strum(serialize = "/lists")]
    Lists,
}

#[derive(Debug, Builder, Default)]
#[builder(setter(into), default)]
pub struct Search {
    #[builder(setter(strip_option))]
    query: Option<String>,
    kind: SearchKind,
    #[builder(default = "0")]
    offset: u32,
    #[builder(default = "10")]
    limit: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SearchResult {
    pub start: u32,
    pub num_found: u32,
    pub docs: Vec<Document>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FieldNamesAsSlice)]
pub struct Document {
    pub key: Key,
    pub cover_edition_key: Option<OlId>,
    pub edition_count: u32,
    pub title: String,
    #[serde(default)]
    pub author_name: Vec<String>,
    pub first_publish_year: Option<u16>,
    pub ratings_average: Option<f64>,
}

impl OpenLibRequest for Search {
    type Result = SearchResult;

    fn path(&self) -> String {
        format!("/search{}.json", self.kind)
    }

    fn query(&self) -> Vec<(&'static str, String)> {
        vec![
            ("q", self.query.as_deref().unwrap_or_default().to_string()),
            ("offset", self.offset.to_string()),
            ("limit", self.limit.to_string()),
            ("fields", Document::FIELD_NAMES_AS_SLICE.join(",")),
        ]
    }
}
