use std::{fmt::Display, str::FromStr};

use anyhow::{bail, Context};
use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    serde_with::SerializeDisplay,
    serde_with::DeserializeFromStr,
)]
pub struct OlId {
    pub id: u64,
    pub kind: OlIdKind,
}

impl Display for OlId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "OL{}{}", self.id, self.kind)
    }
}

impl FromStr for OlId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(rest) = s.strip_prefix("OL") else {
            bail!("missing 'OL' prefix in '{s}'");
        };
        let (id, kind) = rest.split_at(rest.len() - 1);
        let id = id
            .parse::<u64>()
            .with_context(|| format!("invalid numeric part '{id}'"))?;
        let kind = kind
            .parse::<OlIdKind>()
            .with_context(|| format!("invalid ID suffix '{kind}'"))?;
        Ok(Self { id, kind })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::Display, strum::EnumString)]
pub enum OlIdKind {
    #[strum(serialize = "A")]
    Author,
    #[strum(serialize = "M")]
    Book,
    #[strum(serialize = "W")]
    Work,
}

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, Serialize, serde_with::DeserializeFromStr,
)]
pub struct Key {
    pub path: String,
    pub id: OlId,
}

impl Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.path, self.id)
    }
}

impl FromStr for Key {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((path, id)) = s.rsplit_once('/') else {
            bail!("missing path in '{s}'");
        };
        let path = path.to_owned();
        let id = id
            .parse()
            .with_context(|| format!("invalid ID part '{id}'"))?;
        Ok(Self { path, id })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyedKey {
    pub key: Key,
}

impl From<Key> for KeyedKey {
    fn from(key: Key) -> Self {
        Self { key }
    }
}

impl From<KeyedKey> for Key {
    fn from(value: KeyedKey) -> Self {
        value.key
    }
}
