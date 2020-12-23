use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub mod protocol;

#[derive(Debug, Serialize, Deserialize)]
pub enum ProcessletMsg {
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
    TextField {
        current_text: String,
        cursor_idx: usize,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Value {
    Path(PathBuf),
    List(Vec<Value>),
}
