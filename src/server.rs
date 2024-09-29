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

#[derive(Serialize, Deserialize, Debug)]
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
    println!(
        "Server or node {} is starting on {} : File: server.rs",
        node_id, node_address
    );

    loop {
        match listener.accept().await {
            Ok((socket, _)) => {
                let node_id_cloned = node_id.to_string();
                tokio::spawn(async move {
                    if let Err(e) = handle_client(socket, node_id_cloned).await {
                        eprintln!("Error handling client: {}  File: server.rs", e);
                    }
                });
            }
            Err(e) => {
                eprintln!("Failed to accept connection: {}  File: server.rs", e);
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

    //Debug : When a new client connects
    println!("Client connected to node: {}", node_id);

    //getting the size of request from the client's TCP socket stream? and reading it into our created buffer
    let number_of_bytes = socket.read(&mut buffer).await?;
    println!(
        "Received {} bytes from client : File: server.rs",
        number_of_bytes
    );

    // getting the request itself and serializing or deserializing it into Op
    let request: Op = serde_json::from_slice(&buffer[..number_of_bytes])?;
    println!("File: server.rs => Parsed request: {:?} ", request);

    //Processing the request; Now we will include hashing to decide where the data should be written
    let response = match request {
        Op::Read(key) => {
            let result = read_from_node(&key);
            println!(" File: server.rs Reading key: {}", key);
            match result {
                Some(data) => RpcResponse { result: data },
                None => RpcResponse {
                    result: " File: server.rs Key Not Found(404)".to_string(),
                },
            }
        }
        Op::Write(key, data) => {
            println!("Writing data: {:?} to key: {}", data, key);
            write_data_to_node(key.clone(), data);

            RpcResponse {
                result: format!(
                    " File: server.rs Stored data under key '{}' on node {}",
                    key, node_id
                ),
            }
        }
        _ => RpcResponse {
            result: "Unknown Operation".to_string(),
        },
    };

    //Sending the response generated: back to client
    println!("Sending response: {:?}", response);

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
