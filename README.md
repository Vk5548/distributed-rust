# Momento: Simple Distributed System in Rust

This project demonstrates a **distributed key-value store** implemented in **Rust**, using **Tokio** for async networking and **etcd** for node registration and discovery. Each data node stores data locally in memory, and a **central coordinator** (or client-side logic) handles request distribution to the correct node based on **MementoHash**.

## MementoHash Integration

We have integrated the algorithm described in:

> **MementoHash: A Stateful, Minimal Memory, Best Performing Consistent Hash Algorithm**  
> *Massimo Coluzzi, Amos Brocco, Alessandro Antonucci, Tiziano Leidi*  
> Department of Innovative Technologies,  
> University of Applied Sciences and Arts of Southern Switzerland, Lugano, Switzerland

Key points about **MementoHash**:

- **Stateful, Minimal Memory**: Stores a compact “dense b-array” structure, only tracking removed buckets.  
- **Best Performing**: Improves on standard consistent hashing, offering minimal key reassignments and efficient lookups via a Jump Hash–based approach.  
- **Dynamic Node Add/Remove**: Allows new buckets to be added with minimal key movement, and removed nodes are quickly replaced in the b-array.

### Implementation Details

- **`momentohash.rs`**: This file contains the core MementoHash data structure and logic for adding/removing buckets and mapping keys to buckets using Jump Hash.  
- **Unit Tests**: We’ve added tests to validate the correctness of key distribution, node addition, and node removal scenarios. Check the `tests/` folder (or `momentohash.rs` if inline tests) for examples.

## Project Structure

- **`main.rs`** or **`coordinator.rs`** (depending on your final design) orchestrates the system:
  - Registers nodes with etcd.
  - Uses MementoHash to determine which node (bucket) is responsible for a given key.
  - Forwards client requests (read/write) to the correct data node.
- **`server.rs`**: Runs a single data-node instance, which listens on a specified port (`0.0.0.0:8080` inside its Docker container). Each node maintains a local `HashMap` of key-value data.  
- **`client.rs`** or **client_main.rs**: Illustrates how to send read/write operations to the coordinator (or directly to nodes, if you opt for client-side hashing).  
- **`etcd.rs`**: Handles node registration and deregistration with an etcd service.

## What’s Been Accomplished

- **Single Node per Container**: Removed loops that spawned multiple servers in one container. Each container now hosts exactly one node.  
- **MementoHash Integration**: Created `momentohash.rs` to implement the dense b-array approach described in the paper, with Jump Hash for bucket lookups.  
- **Unit Testing**: Wrote tests to verify correctness of add/remove logic, ensuring minimal disruption when new nodes come online or existing nodes fail.  
- **etcd Integration**: Each node registers its address in etcd, so the coordinator or clients can dynamically discover active nodes.

## Future Work

- **Automatic Detection of Node Failures**: Implement a health-check mechanism so the coordinator can remove inactive nodes from MementoHash.  
- **Replication**: For higher availability, replicate data across multiple nodes (buckets) rather than storing each key on just one node.  
- **Performance Tuning**: Benchmark and optimize throughput, especially under high concurrency.

---
