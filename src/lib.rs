use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub enum ClientMsg {
    PressedKey(char),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ServerMsg {
    NewText(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ProcessletMsg {
    NewOutput(Value),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Value {
    Path(PathBuf),
    List(Vec<Value>),
}
