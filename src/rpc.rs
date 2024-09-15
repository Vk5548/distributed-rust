use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Op {
    Read(String),
    Write(String, Vec<u8>), //Updated to include the key
    Stat(String),
    Delete(String),
}
