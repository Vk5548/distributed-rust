//custom-created crates
// This will be removed because momentoHash will be implemented
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

// Now we don't need separate data stores here as each server container will have its own data structure
lazy_static! {
    static ref DATA_STORE: Mutex<HashMap<String, Vec<u8>>> = Mutex::new(HashMap::new());
}

pub async fn start_server(
    node_id: &str,
    node_address: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    //Now binding to the specific server container
    let listener = TcpListener::bind(node_address).await?;
    println!("Server or node {} is starting on {}", node_id, node_address);

    loop {
        match listener.accept().await {
            Ok((socket, _)) => {
                let node_id_cloned = node_id.to_string();
                tokio::spawn(async move {
                    if let Err(e) = handle_client(socket, node_id_cloned).await {
                        eprintln!("Error handling client: {}", e);
                    }
                });
            }
            Err(e) => {
                eprintln!("Failed to accept connection: {}", e);
            }
        }
    }
}

//Function to handle individual client connections; meaning for each client connection, this will be called
async fn handle_client(
    mut socket: tokio::net::TcpStream,
    node_id: String,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut buffer = vec![0; 1024];

    //getting the size of request from the client's TCP socket stream? and reading it into our created buffer
    let number_of_bytes = socket.read(&mut buffer).await?;
    // getting the request itself and serializing it into Op
    let request: Op = serde_json::from_slice(&buffer[..number_of_bytes])?;

    //Processing the request; Now we will include hashing to decide where the data should be written
    let response = match request {
        Op::Read(key) => {
            let result = read_from_node(&key);

            match result {
                Some(data) => RpcResponse { result: data },
                None => RpcResponse {
                    result: "Key Not Found(404)".to_string(),
                },
            }
        }
        Op::Write(key, data) => {
            write_data_to_node(key.clone(), data);

            RpcResponse {
                result: format!("Stored data under key '{}' on node {}", key, node_id),
            }
        }
        _ => RpcResponse {
            result: "Unknown Operation".to_string(),
        },
    };

    //Sending the response generated: back to client
    let response_json = serde_json::to_string(&response)?;
    socket.write_all(response_json.as_bytes()).await?;

    Ok(())
}

//Helper functions to interact with data node
fn read_from_node(key: &str) -> Option<String> {
    let store = DATA_STORE.lock().unwrap();
    let result = store
        .get(key)
        .map(|val| format!("Data for key {} is {:?}", key, val));
    result
}

fn write_data_to_node(key: String, data: Vec<u8>) {
    let mut store = DATA_STORE.lock().unwrap();
    store.insert(key, data);
}
