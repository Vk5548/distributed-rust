// To be included in the module tree, everything needs to be included in main.rs?
// mod client;
// mod hash;
// mod rpc;
// mod server;

//custom-defined
use simple_distribution_system::etcd;
use simple_distribution_system::server;

// imported from pre-existing libraries
use std::env;
// use tokio::task; // do not need it here for now

#[tokio::main]
async fn main() {
    //DOESN"T make sense, each cob=ntainer is now staring 3 containers
    //Getting the number of servers to start from environment varibles or default to 3?
    // let num_servers: usize = env::var("NUM_SERVERS")
    //     .unwrap_or("3".to_string())
    //     .parse()
    //     .expect("NUM_SREVERS must be a valid number");
    // println!("Starting the RPC server");

    //Dynamically starting the multiple servers and registering them with etcd
    // servers are tasks here?

    // Reading server number from , env varible through std
    let node_id = env::var("NODE_ID").unwrap_or("DataNodeX".to_string());
    let node_address = env::var("NODE_ADDRESS").unwrap_or("0.0.0.0:8080".to_string());

    // 1. Register node with etcd
    if let Err(e) = etcd::register_node_with_etcd(&node_id, &node_address).await {
        eprintln!(
            "Failed to Register node {} in main.rs Err(e) = {:?}",
            node_id, e
        );
        return;
    }

    //Start the server for this container
    if let Err(e) = server::start_server(&node_id, &node_address).await {
        eprintln!(
            "Failed to Start the server node {} in main.rs Err(e) = {:?}",
            node_id, e
        );
    }
}
