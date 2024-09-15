// To be included in the module tree, everything needs to be included in maain.rs?
mod client;
mod hash;
mod rpc;
mod server;

use rpc::Op;
use tokio::task;

#[tokio::main]
async fn main() {
    println!("Starting the RPC server");

    // Starting the server in a seperate task
    task::spawn(async {
        server::start_server().await;
        println!("RPC server started");
    });

    //giving some time for server to start
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    //Sending some client requests: Updated to test the key and val sent from the client
    client::send_request(Op::Read("foo".to_string())).await;
    client::send_request(Op::Write("write_to_a?".to_string(), vec![1, 2, 3, 4])).await;
    client::send_request(Op::Write("write_to_a?".to_string(), vec![4, 6, 8, 4])).await;
    client::send_request(Op::Write("write_to_b?".to_string(), vec![10, 12, 13, 14])).await;
    client::send_request(Op::Write("write_to_c?".to_string(), vec![13, 32, 33, 34])).await;
    client::send_request(Op::Read("write_to_c?".to_string())).await;
    client::send_request(Op::Read("write_to_b?".to_string())).await;
    client::send_request(Op::Read("write_to_a?".to_string())).await;
}
