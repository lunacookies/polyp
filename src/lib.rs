use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub enum UserInput {
    PressedKey(Key),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Key {
    Char(char),
    Backspace,
    Up,
    Down,
    Left,
    Right,
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
