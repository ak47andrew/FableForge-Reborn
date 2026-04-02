use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use serde_json::Error;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case", content = "data")]
pub enum CSPacket {
    AddToken { token: TokenNetwork },
    MoveToken { token: TokenNetwork },
    DeleteToken { token: u32 },
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "snake_case", content = "data")]
pub enum SCPacket {
    Ok,
    AddToken { token: TokenNetwork },
    MoveToken { token: TokenNetwork },
    DeleteToken { token: u32 },
}

#[derive(Deserialize, Serialize, Debug, Copy, Clone)]
pub struct TokenNetwork {
    pub id: u32,
    pub pos: Vector2D
}

#[derive(Deserialize, Serialize, Debug, Copy, Clone)]
pub struct Vector2D {
    pub x: f32,
    pub y: f32,
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
