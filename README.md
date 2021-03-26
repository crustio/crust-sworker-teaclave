# Crust Sworker Teaclave

## Prerequisites:
- Hardware requirements: 
  CPU must contain **SGX module**, and make sure the SGX function is turned on in the bios, please click [this page](https://github.com/crustio/crust/wiki/Check-TEE-supportive) to check if your machine supports SGX
  
- Other configurations
  - **Secure Boot** in BIOS needs to be turned off
  - Need use ordinary account, **cannot support root account**

- Ensure that you have one of the following required operating systems:
  * Ubuntu\* 16.04 LTS Desktop 64bits (just for docker mode)
  * Ubuntu\* 16.04 LTS Server 64bits (just for docker mode)
  * Ubuntu\* 18.04 LTS Desktop 64bits 
  * Ubuntu\* 18.04 LTS Server 64bits 

- SGX Driver
  * Install SGX driver[Intel SGX driver 2.9.1 for Linux](https://01.org/intel-software-guard-extensions/downloads)

- Clone project
  ```
  git clone https://github.com/crustio/crust-sworker-teaclave.git
  ```

## Build

### Build from docker
use '**sudo ./docker/build.sh**' to build docker

### Build from source
Please refer to [this page](https://github.com/apache/incubator-teaclave-sgx-sdk#native-without-docker-not-recommended)

## Run

### Run from docker
    ```
    cd docker/build
    sudo docker-compose up -d crust-sworker-teaclave
    ```

### Run from source
run '**sudo ./scripts/install.sh**', this command will install crust-sworker-teaclave to **/opt/crust/crust-sworker**.
Then run '**/opt/crust/crust-sworker/bin/crust-sworker-t**' to start crust-sworker
