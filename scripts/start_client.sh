#!/bin/bash

# Build the client image
docker build --no-cache -t client -f ./Client_Dockerfile .

# Run the containers on the same network as the containers
docker run --network my-distributed-system-network --name client client