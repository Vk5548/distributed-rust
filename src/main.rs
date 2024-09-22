// To be included in the module tree, everything needs to be included in main.rs?
// mod client;
// mod hash;
// mod rpc;
// mod server;

use simple_distribution_system::server;

#[tokio::main]
async fn main() {
    println!("Starting the RPC server");

    // Starting the server (not in a seperate task anymore) :
    // No need to start on sepearte task, because starting server will be the
    // only task that will run

    server::start_server().await;
}
