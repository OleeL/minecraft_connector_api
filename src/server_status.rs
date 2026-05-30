use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerStatus {
    pub version: Version,
    pub players: Players,
    pub description: Description,
    pub favicon: String,
    pub modinfo: Option<Modinfo>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Version {
    pub name: String,
    pub protocol: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Players {
    pub max: i64,
    pub online: i64,
    pub sample: Option<Vec<Sample>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sample {
    pub name: String,
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Description {
    pub text: String,
    pub extra: Option<Vec<Extra>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Extra {
    pub extra: Option<Vec<Extra>>,
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Modinfo {
    #[serde(rename = "type")]
    pub type_field: String,
    pub mod_list: Vec<Value>,
}
