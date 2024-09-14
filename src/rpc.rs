use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Op{
    Read(String),
    Write(Vec<u8>),
    Stat(String),
    Delete(String)
}