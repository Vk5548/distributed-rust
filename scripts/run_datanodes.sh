#!/bin/bash

# Step 1: Clean up the exited containers:
docker ps -a --filter "status=exited" --filter "name=datanode" -q | xargs -r docker rm 

# Step 2: building the images:
docker build -t datanode1 .
docker build -t datanode2 .
docker build -t datanode3 .

# Step 3: Craete a network if it doesn't exist
# docker network inspect my-distributed-system-network >/dev/null 2>&1 || docker network create my-distributed-system-network

# Step 4: Run the conatiners finally:
docker run -d --name datanode1 --network my-distributed-system-network -p 8081:8080 -e NODE_ID="DataNode1" -e NODE_ADDRESS="127.0.0.1:8081" datanode1
docker run -d --name datanode2 --network my-distributed-system-network -p 8082:8080 -e NODE_ID="DataNode2" -e NODE_ADDRESS="127.0.0.1:8082" datanode2
docker run -d --name datanode3 --network my-distributed-system-network -p 8083:8080 -e NODE_ID="DataNode3" -e NODE_ADDRESS="127.0.0.1:8083" datanode3
