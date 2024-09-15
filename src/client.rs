use crate::rpc::Op;
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[derive(Serialize, Deserialize, Debug)]
struct RpcResponse {
    result: String,
}

pub async fn send_request(op: Op) {
    let mut socket = TcpStream::connect("127.0.0.1:8080").await.unwrap();

    //Serialize the operation and send it to the server
    let request_json = serde_json::to_string(&op).unwrap();
    socket.write_all(request_json.as_bytes()).await.unwrap();

    //Read the response from the server
    let mut buffer = vec![0; 1024];
    let number_of_bytes = socket.read(&mut buffer).await.unwrap();
    let response: RpcResponse = serde_json::from_slice(&buffer[..number_of_bytes]).unwrap();

    println!("Response from server {:?}", response);
}
