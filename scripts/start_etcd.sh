#!/bin/bash

#Pull and run the etcd docker container
docker run -d --name etcd-server \
 -p 2379:2379 \
 -p 2380:2380 \
 quay.io/coreos/etcd:v3.5.16 \
 etcd --name etcd0 \
 --data-dir /etcd-data --listen-client-urls http://0.0.0.0:2379 \
 --advertise-client-urls http://0.0.0.0:2379 

 echo "etcd is running on localhost:2379"

