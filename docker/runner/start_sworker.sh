#!/bin/bash
crustdir=/opt/crust
version=$(cat /crust-sworker/VERSION | head -n 1)
crustsworkerdir=$crustdir/crust-sworker
inteldir=/opt/intel
wait_time=10

echo "Starting curst sworker $version"
echo "Wait $wait_time seconds for aesm service fully start"
NAME=aesm_service AESM_PATH=/opt/intel/sgx-aesm-service/aesm LD_LIBRARY_PATH=/opt/intel/sgx-aesm-service/aesm /opt/intel/sgx-aesm-service/aesm/aesm_service
sleep $wait_time

ps -ef | grep aesm

echo "Run sworker with arguments: $ARGS"
/opt/crust/crust-sworker/bin/crust-sworker-t $ARGS
