#!/bin/bash

#create a custom Docker network if itdoesn't already exist
docker network inspect my-distributed-system-network >/dev/null 2>&1 || \
docker network create my-distributed-system-network

#Pull and run the etcd docker container
docker run -d --name etcd-server \
 --network my-distributed-system-network \
 -p 2379:2379 \
 -p 2380:2380 \
 quay.io/coreos/etcd:v3.5.16 \
 etcd --name etcd0 \
 --data-dir /etcd-data --listen-client-urls http://0.0.0.0:2379 \
 --advertise-client-urls http://etcd-server:2379

 echo "etcd is running on the docker network 'my-distributed-system-network' and accessible at localhost:2379 or etcd-server:2379"

