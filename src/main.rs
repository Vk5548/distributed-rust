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
use tokio::signal;
use tokio::task;

#[tokio::main]
async fn main() {
    //Getting the number of servers to start from environment varibles or default to 3?
    let num_servers: usize = env::var("NUM_SERVERS")
        .unwrap_or("3".to_string())
        .parse()
        .expect("NUM_SREVERS must be a valid number");
    println!("Starting the RPC server");

    //Dynamically starting the multiple servers and registering them with etcd
    let mut tasks = vec![];

    for i in 0..num_servers {
        // generating unique node_id and addressess(port) for each server
        let node_id = format!("DataNode{}", i + 1);
        let node_address = format!("127.0.0.1:{}", 8081 + i);

        //Spawning a sperate task for each server;
        let task = task::spawn(async move {
            //Register the node with etcd
            if let Err(e) = etcd::register_node_with_etcd(&node_id, &node_address).await {
                eprintln!(
                    "Failed to register the node: {} with the etcd container: {}",
                    node_id, e
                );
                return;
            }

            println!(
                "Node {}registered with etcd container successfully ",
                node_id
            );

            //Now starting the server so it runs indefinetely
            println!(
                "Starting the srever for node: {} at this address : {}",
                node_id, node_address
            );
            if let Err(e) = server::start_server(&node_id, &node_address).await {
                eprintln!("Server {} failed: {}", node_id, e);
            }
        });
        //pushing all the servers(tasks in the array and awaiting them)
        tasks.push(task);
    }

    // Waiting for all servers to start indefinetely
    for task in tasks {
        if let Err(e) = task.await {
            eprintln!("A server task failed: {}", e);
        }
    }
}
