use std::{
    borrow::Cow,
    collections::{BTreeMap, BTreeSet, HashMap},
    time::Duration,
};

use chrono::NaiveDate;
use openlibrsry::OlId;
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
    /// map of Open Library id to Book structs
    #[serde(default)]
    pub books: HashMap<OlId, Book>,
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

const fn default_watch_speed() -> f32 {
    1.
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Rating {
    pub date: NaiveDate,

    /// Integer rating in range `1..=10`
    pub rating: u8,

    /// The watch speed, usually in range `1.0..=5.0`
    #[serde(default = "default_watch_speed")]
    pub speed: f32,

    pub platform: Option<Platform>,
    #[serde(default)]
    pub tags: BTreeSet<u32>,
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
    #[serde(rename = "DVD")]
    #[strum(serialize = "DVD")]
    Dvd,
    BluRay,
    Cinema,
    #[serde(rename = "TV")]
    #[strum(serialize = "TV")]
    Tv,
    Airplane,
    #[serde(rename = "Apple TV")]
    #[strum(serialize = "Apple TV")]
    AppleTv,
    Stan,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Book {
    pub olid: OlId,
    pub isbn: u64,
    pub title: String,
    pub description: String,
    pub authors: Vec<String>,
    pub readings: Vec<Reading>,
    pub tags: BTreeSet<u32>,
    pub release_date: String,
    pub start_page: u16,
    pub end_page: u16,
    pub score: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Reading {
    pub pages_read: BTreeMap<NaiveDate, u16>,
    pub rating: Option<Rating>,
}
