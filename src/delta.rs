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
pub enum Attribute {
    Bold(bool),
    Italic(bool),
    Header(u8),
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
