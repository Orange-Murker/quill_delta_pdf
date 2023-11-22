use std::str::FromStr;

use serde::Deserialize;
use serde_with::{serde_as, EnumMap};
use url::Url;

#[derive(Deserialize, Debug)]
pub struct Image {
    pub image: Url,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum DeltaType {
    String(String),
    Image(Image),
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ListType {
    Bullet,
    Ordered,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Attribute {
    Bold(bool),
    Italic(bool),
    Header(u8),
    List(ListType),
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Change {
    Insert(DeltaType),
    Delete(DeltaType),
    Retain(DeltaType),
}

#[serde_as]
#[derive(Deserialize, Debug)]
pub struct Op {
    #[serde(flatten)]
    pub change: Change,
    #[serde_as(as = "Option<EnumMap>")]
    pub attributes: Option<Vec<Attribute>>,
}

#[derive(Deserialize, Debug)]
pub struct Delta {
    pub ops: Vec<Op>,
}

impl Delta {
    /// Creates an empty Delta
    pub fn new() -> Self {
        Self { ops: Vec::new() }
    }

    /// Add a new Op to the Delta
    pub fn push(&mut self, op: Op) {
        self.ops.push(op);
    }

    /// Extend one Delta with another
    pub fn extend(&mut self, other: Delta) {
        self.ops.extend(other.ops);
    }
}

impl FromStr for Delta {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}
