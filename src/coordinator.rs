//use tonic instead of reqwest
use tonic::{transport::Server, Request, Response, Status, Streaming};
// etcd_client is a gRPC client for etcd
use etcd_client::{Client, Event, WatchOptions};


//importng existing crates to which cooedinator communicates
// use crate::etcd;
use crate::momentohash::momentohash::MomentoHash;
// use crate::rpc::Op;
use grpc::coordinator::{
    coordinator_server::{Coordinator, CoordinatorServer}, //grpc auto-generated trait
    LookUpRequest, LookUpResponse, NodeUpdate, Empty};
// Importing Server Response Structure (Assumed from `server.rs`)
use crate::server::RpcResponse;


//importing data structures and utilities that will be needed to handle the data and async handling
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::{RwLock, mpsc};
use tokio_stream::wrappers::ReceiverStream;



///Struct to represent the Coordinator
pub struct CoordinatorService {
    momentoHash: Arc<RwLock<MomentoHash>>, //MomentoHash instance
    node_bucket_map: Arc<RwLock<HashMap<usize, String>>>, //Bucket ID → DataNode mapping(datanodeX);
    etcd_client: Client,                              //EtcdClient instance
}


#[tonic::async_trait]
impl Coordinator for CoordinatorService{
    ///gRPC method to find the DataNode for a given key
    async fn get_data_node(
        &self, 
        request: Request<LookUpRequest>,
    ) -> Result<Response<LookUpResponse>, Status> {
        let key = request.into_inner().key; // into_inner() is used to extract the request payload and consumes it

        //Step 1: Getting the bucket ID for the given key from the MomentoHash
        let bucket = self.momentoHash
    }
}