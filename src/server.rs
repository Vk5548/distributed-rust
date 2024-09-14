use crate::rpc::Op;
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[derive(Serialize, Deserialize)]
struct RpcResponse {
    result: String,
}

pub async fn start_server() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    println!("Server is starting on 127.0.0.1:8080");

    loop {
        let (mut socket, _) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            let mut buffer = vec![0; 1024];
            //getting the size of request
            let numberOfBytes = socket.read(&mut buffer).await.unwrap();
            // getting the request itself
            let request: Op = serde_json::from_slice(&buffer[..numberOfBytes]).unwrap();

            //Processing the request
            let response = match request {
                Op::Read(key) => {
                    //For now, just echoing it
                    RpcResponse {
                        result: format!("Read {}", key),
                    }
                }
                Op::Write(data) => RpcResponse {
                    result: format!("Written these {} bytes", data.len()),
                },
                _ => RpcResponse {
                    result: "Unknown Operation".to_string(),
                },
            };

            //Sending the response generated:
            let response_json = serde_json::to_string(&response).unwrap();
            socket.write_all(response_json.as_bytes()).await.unwrap();
        });
    }
}
