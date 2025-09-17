use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerStatus {
    pub version: Version,
    pub players: Players,
    pub description: ChatComponent,
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatComponent {
    Text(String),
    Object(ChatObject),
    Array(Vec<ChatComponent>),
}

impl Default for ChatComponent {
    fn default() -> Self {
        ChatComponent::Text(String::new())
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatObject {
    #[serde(default)]
    pub text: String,
    #[serde(default)]
    pub extra: Vec<ChatComponent>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Modinfo {
    #[serde(rename = "type")]
    pub type_field: String,
    pub mod_list: Vec<Value>,
}
