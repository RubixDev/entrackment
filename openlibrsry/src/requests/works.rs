use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use crate::{olid::KeyedKey, Key, OlId, OpenLibRequest};

#[derive(Debug, Builder)]
#[builder(setter(into))]
pub struct WorksEditions {
    id: OlId,
    #[builder(default = "0")]
    offset: u32,
    #[builder(default = "10")]
    limit: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WorksEditionsResult {
    pub links: WorksEditionsLinks,
    pub size: u32,
    pub entries: Vec<Edition>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WorksEditionsLinks {
    #[serde(rename = "self")]
    pub self_: String,
    pub work: String,
    pub next: Option<String>,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edition {
    pub key: Key,
    #[serde(default)]
    pub isbn_13: Vec<String>,
    #[serde(default)]
    pub isbn_10: Vec<String>,
    pub title: String,
    #[serde_as(deserialize_as = "Option<serde_with::FromInto<Description>>")]
    pub description: Option<String>,
    #[serde(default)]
    #[serde_as(deserialize_as = "Vec<serde_with::FromInto<KeyedKey>>")]
    pub authors: Vec<Key>,
    pub publish_date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Description {
    Direct(String),
    Typed {
        #[serde(rename = "type")]
        r#type: String,
        value: String,
    },
}

impl From<Description> for String {
    fn from(value: Description) -> Self {
        match value {
            Description::Direct(value) => value,
            Description::Typed { r#type: _, value } => value,
        }
    }
}

impl From<String> for Description {
    fn from(value: String) -> Self {
        Self::Direct(value)
    }
}

impl OpenLibRequest for WorksEditions {
    type Result = WorksEditionsResult;

    fn path(&self) -> String {
        format!("/works/{}/editions.json", self.id)
    }

    fn query(&self) -> Vec<(&'static str, String)> {
        vec![
            ("offset", self.offset.to_string()),
            ("limit", self.limit.to_string()),
        ]
    }
}
