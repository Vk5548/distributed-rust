//author created crates
use crate::hash::{hash_key_to_node, DataNode};
use crate::rpc::Op;

//std available crates
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[derive(Serialize, Deserialize)]
struct RpcResponse {
    result: String,
}

// Shared state across the project;
// It changes for each available data store or data node we hvae
lazy_static! {
    static ref DATA_STORE_A: Mutex<HashMap<String, Vec<u8>>> = Mutex::new(HashMap::new());
    static ref DATA_STORE_B: Mutex<HashMap<String, Vec<u8>>> = Mutex::new(HashMap::new());
    static ref DATA_STORE_C: Mutex<HashMap<String, Vec<u8>>> = Mutex::new(HashMap::new());
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

//Function to handle individual client connections; meaning for each client connection, this will be called
async fn handle_client(mut socket: tokio::net::TcpStream) {
    let mut buffer = vec![0; 1024];

    //getting the size of request from the client's TCP socket stream? and reading it into our created buffer
    let number_of_bytes = socket.read(&mut buffer).await.unwrap();
    // getting the request itself and serializing it into Op
    let request: Op = serde_json::from_slice(&buffer[..number_of_bytes]).unwrap();

    //Processing the request; Now we will include hashing to decide where the data should be written
    let response = match request {
        Op::Read(key) => {
            let data_node = hash_key_to_node(&key);
            let result = match data_node {
                DataNode::A => read_from_node(&DATA_STORE_A, &key),
                DataNode::B => read_from_node(&DATA_STORE_B, &key),
                DataNode::C => read_from_node(&DATA_STORE_C, &key),
            };

            match result {
                Some(data) => RpcResponse { result: data },
                None => RpcResponse {
                    result: "Key Not Found(404)".to_string(),
                },
            }
        }
        Op::Write(key, data) => {
            //Modified to receive key from the client
            let data_node = hash_key_to_node(&key);
            match data_node {
                DataNode::A => write_data_to_node(&DATA_STORE_A, key.clone(), data),
                DataNode::B => write_data_to_node(&DATA_STORE_B, key.clone(), data),
                DataNode::C => write_data_to_node(&DATA_STORE_C, key.clone(), data),
            }

            RpcResponse {
                result: format!("Stored data under key '{}' on node {:?}", key, data_node),
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

//Helper functions to interact with data node
fn read_from_node(store: &Mutex<HashMap<String, Vec<u8>>>, key: &str) -> Option<String> {
    let store = store.lock().unwrap();
    let result = store
        .get(key)
        .map(|val| format!("Data for key {} is {:?}", key, val));
    result
}

fn write_data_to_node(store: &Mutex<HashMap<String, Vec<u8>>>, key: String, data: Vec<u8>) {
    let mut store = store.lock().unwrap();
    store.insert(key, data);
}
