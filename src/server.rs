use crate::rpc::Op;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::format;
use std::sync::Mutex;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[derive(Serialize, Deserialize)]
struct RpcResponse {
    result: String,
}

// Shared state across the project;
lazy_static! {
    static ref DATA_STORE: Mutex<HashMap<String, Vec<u8>>> = Mutex::new(HashMap::new());
}

pub async fn start_server() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Server is starting on 127.0.0.1:8080");

    loop {
        let (socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            handle_client(socket).await;
        });
    }
}

async fn handle_client(mut socket: tokio::net::TcpStream) {
    let mut buffer = vec![0; 1024];
    //getting the size of request form the client's TCP socket stream?
    let number_of_bytes = socket.read(&mut buffer).await.unwrap();
    // getting the request itself and serializing it into Op
    let request: Op = serde_json::from_slice(&buffer[..number_of_bytes]).unwrap();

    //Processing the request
    let response = match request {
        Op::Read(key) => {
            let store = DATA_STORE.lock().unwrap();
            let result = store
                .get(&key)
                .map(|val| format!("Data for key {} is {:?}", key, val))
                .unwrap_or("Key not found".to_string());
            RpcResponse { result }
        }
        Op::Write(data) => {
            let key = "default_key".to_string();
            let mut store = DATA_STORE.lock().unwrap();
            let len = data.len();
            store.insert(key.clone(), data);
            RpcResponse {
                result: format!("Stored these {} bytes of data under this {}", len, key),
            }
        }
        _ => RpcResponse {
            result: "Unknown Operation".to_string(),
        },
    };

    //Sending the response generated:
    let response_json = serde_json::to_string(&response).unwrap();
    socket.write_all(response_json.as_bytes()).await.unwrap();
}
