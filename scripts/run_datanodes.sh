#!/bin/bash

# Step 1: Clean up the exited containers:
docker ps -a --filter "status=exited" --filter "name=datanode" -q | xargs -r docker rm 

# Step 2: building the images:
docker build --no-cache -t datanode -f ./Dockerfile .
# docker build -t datanode2 -f ../Dockerfile .
# docker build -t datanode3 -f ../Dockerfile .

# Step 3: Craete a network if it doesn't exist
# docker network inspect my-distributed-system-network >/dev/null 2>&1 || docker network create my-distributed-system-network

# Step 4: Run the conatiners finally:
# docker run -d --name datanode1 --network my-distributed-system-network -p 8084:8080 -e NODE_ID="DataNode1" -e NODE_ADDRESS="0.0.0.0:8080" datanode
# docker run -d --name datanode2 --network my-distributed-system-network -p 8085:8080 -e NODE_ID="DataNode2" -e NODE_ADDRESS="0.0.0.0:8080" datanode
# docker run -d --name datanode3 --network my-distributed-system-network -p 8086:8080 -e NODE_ID="DataNode3" -e NODE_ADDRESS="0.0.0.0:8080" datanode


# Step 4: Run the server containers in the custom network; when client also runs on container
docker run -d --network my-distributed-system-network --name datanode1 -e NODE_ID="DataNode1" -e NODE_ADDRESS="0.0.0.0:8080" datanode
docker run -d --network my-distributed-system-network --name datanode2 -e NODE_ID="DataNode2" -e NODE_ADDRESS="0.0.0.0:8080" datanode
docker run -d --network my-distributed-system-network --name datanode3 -e NODE_ID="DataNode3" -e NODE_ADDRESS="0.0.0.0:8080" datanode