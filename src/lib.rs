use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub enum UserInput {
    PressedKey(char),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ServerMsg {
    NewText(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Ui {
    Value(Value),
    TextField { current_text: String },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Value {
    Path(PathBuf),
    List(Vec<Value>),
}
