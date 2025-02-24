use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum Op {
    Read(String),
    Write(String, Vec<u8>), //Updated to include the key
    Stat(String),
    Delete(String),
}

// Won't be used since I am using gRPC instead of HTTP
//with Flatbuffers instead of JSON and protobuf
