use simple_distribution_system::client;
use simple_distribution_system::rpc;

use rpc::Op;

#[tokio::main]
async fn main() {
    // Sending some client requests: Updated to test the key and val sent from the client
    client::send_request(Op::Read("foo".to_string())).await;
    client::send_request(Op::Write("write_to_a?".to_string(), vec![1, 2, 3, 4])).await;
    client::send_request(Op::Write("write_to_a?".to_string(), vec![4, 6, 8, 4])).await;
    client::send_request(Op::Write("write_to_b?".to_string(), vec![10, 12, 13, 14])).await;
    client::send_request(Op::Write("write_to_c?".to_string(), vec![13, 32, 33, 34])).await;
    client::send_request(Op::Read("write_to_c?".to_string())).await;
    client::send_request(Op::Read("write_to_b?".to_string())).await;
    client::send_request(Op::Read("write_to_a?".to_string())).await;
}
