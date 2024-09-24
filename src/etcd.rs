//Will manage the node registartion and deristration
// extern crate reqwest;
use reqwest::Client;
//as I am using etcd v3 instead of v2
use base64::{engine::general_purpose, Engine};
use serde::{Deserialize, Serialize};
use std::{error::Error, fmt::format};

#[derive(serde::Serialize)]
struct EtcdPutRequest {
    key: String,
    value: String,
}

#[derive(Serialize)]
struct EtcdDeleteRequest {
    key: String,
}

// Function to register node with etcd
pub async fn register_node_with_etcd(node_id: &str, address: &str) -> Result<(), Box<dyn Error>> {
    let etcd_url = "http://etcd-server:2379/v3/kv/put";

    //Base64 encode the key and value as base64 uses encoding
    let key = general_purpose::STANDARD.encode(format!("/nodes/{}", node_id));
    let value = general_purpose::STANDARD.encode(address);

    // generating payload using the struct

    let payload = EtcdPutRequest { key, value };

    //Creating the new client request
    let client = Client::new();

    // json(&payload) serilaizes the payload automatically because the struct impleme ts teh trait of serde::Serialize: Cool; hun!
    let res = client.post(etcd_url).json(&payload).send().await?;

    if res.status().is_success() {
        println!("Node {} successfully registered with the etcd", node_id);
    } else {
        println!(
            "Failed to register the Node {} with etcd : {:?}",
            node_id,
            res.text().await?
        );
    }

    Ok(())
}

pub async fn deregister_node_from_etcd(node_id: &str) -> Result<(), Box<dyn Error>> {
    let etcd_url = "http://etcd-server:2379/v3/kv/deleterange";

    //Prepare the paylaod for Etcd delete request
    let payload = EtcdDeleteRequest {
        key: general_purpose::STANDARD.encode(format!("/nodes/{}", node_id)),
    };

    let client = Client::new();
    let res = client.post(etcd_url).json(&payload).send().await?;

    if res.status().is_success() {
        println!("Node {} deregistered from etcd", node_id);
    } else {
        println!(
            "Failed to deregister node {} from etcd: {:?}",
            node_id,
            res.text().await?
        );
    }

    Ok(())
}
