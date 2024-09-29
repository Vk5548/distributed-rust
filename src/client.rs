use crate::rpc::Op;
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[derive(Serialize, Deserialize, Debug)]
struct RpcResponse {
    result: String,
}

pub async fn send_request(op: Op) {
    //  bcz external ports are 8081, 8082, 8083
    // If the client runs on host machine
    println!("Attempting to connect to the server at datanode1:8080");
    // let mut socket = TcpStream::connect("127.0.0.1:8084").await.unwrap();

    // If the client runs on a seperate container
    let mut socket = TcpStream::connect("datanode1:8080").await.unwrap();
    // let mut socket = TcpStream::connect("datanode2:8080").await.unwrap();
    // let mut socket = TcpStream::connect("datanode3:8080").await.unwrap();

    //Serialize the operation and send it to the server
    let request_json = serde_json::to_string(&op).unwrap();
    socket.write_all(request_json.as_bytes()).await.unwrap();

    //Read the response from the server
    let mut buffer = vec![0; 1024];
    let number_of_bytes = socket.read(&mut buffer).await.unwrap();
    let response: RpcResponse = serde_json::from_slice(&buffer[..number_of_bytes]).unwrap();

    println!("Response from server {:?}", response);
}
