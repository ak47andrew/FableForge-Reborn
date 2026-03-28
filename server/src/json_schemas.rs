use serde::Deserialize;
use serde_json::Error;

#[derive(Deserialize, Debug)]
#[serde(tag = "request_type", rename_all = "snake_case", content = "data")]
pub enum ReqType {
    Token,
    Move { x: f64, y: f64 },
}

pub fn parse(text: &str) -> Result<ReqType, Error> {
    serde_json::from_str(text)
}