use std::{
    borrow::Cow,
    collections::{BTreeMap, BTreeSet, HashMap},
    time::Duration,
};

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AppData {
    /// map of TMDB id to Movie structs
    #[serde(default)]
    pub movies: BTreeMap<u64, Movie>,
    /// map of tag id to tag structs
    #[serde(default)]
    pub tags: HashMap<u32, Tag>,
    /// map of TMDB or IMDb id to raw movies
    #[serde(default)]
    pub tmdb_cache: HashMap<String, Movie>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Movie {
    pub imdb_id: Option<u32>,
    pub tmdb_id: u64,
    pub title: String,
    pub description: String,
    pub ratings: Vec<Rating>,
    pub tags: BTreeSet<u32>,
    pub platforms: BTreeSet<Platform>,
    pub poster: Option<String>,
    pub release_date: NaiveDate,
    pub runtime: Duration,
    pub score: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MovieStub {
    pub tmdb_id: u64,
    pub title: String,
    pub description: String,
    pub release_date: NaiveDate,
    pub poster: Option<String>,
}

pub type Color = (u8, u8, u8);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Tag {
    pub id: u32,
    pub name: String,
    pub color: Color,
    pub icon: Option<Cow<'static, str>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rating {
    // TODO: bounded u8 or enum?
    pub rating: u8,
    pub date: NaiveDate,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    strum::Display,
    strum::EnumIter,
    strum::FromRepr,
)]
#[repr(u8)]
pub enum Platform {
    #[serde(rename = "Disney+")]
    #[strum(serialize = "Disney+")]
    DisneyPlus,
    Jellyfin,
    Netflix,
    #[serde(rename = "Prime Video")]
    #[strum(serialize = "Prime Video")]
    PrimeVideo,
    YouTube,
}
