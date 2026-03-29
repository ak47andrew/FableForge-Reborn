use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use serde_json::Error;

#[derive(Deserialize, Serialize, Debug, Copy, Clone)]
#[serde(tag = "type", rename_all = "snake_case", content = "data")]
pub enum CSPacket {
    Token,
    Move { x: f32, y: f32 },
}

#[derive(Deserialize, Serialize, Debug, Copy, Clone)]
#[serde(tag = "type", rename_all = "snake_case", content = "data")]
pub enum SCPacket {
    Ok,
    TokenPos {x: f32, y: f32},
}

pub fn parse<T>(text: &str) -> Result<T, Error>
    where T: DeserializeOwned
{
    serde_json::from_str(text)
}

pub fn dump<T>(obj: T) -> String
    where T: Serialize
{
    serde_json::to_string(&obj).unwrap()
}