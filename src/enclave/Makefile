include ../../buildenv.mk

Rust_Enclave_Name := libenclave.a
Rust_Enclave_Files := $(wildcard src/*.rs)
Rust_Target_Path := /opt/intel/incubator-teaclave-sgx-sdk/xargo

ifeq ($(MITIGATION-CVE-2020-0551), LOAD)
export MITIGATION_CVE_2020_0551=LOAD
else ifeq ($(MITIGATION-CVE-2020-0551), CF)
export MITIGATION_CVE_2020_0551=CF
endif

Sgx_Library_Name := lib$(shell cat Cargo.toml | grep "\[lib\]" -A 3 | grep -Po '(?<== ).*' | head -n 1 | sed 's/\"//g').a

.PHONY: all

all: $(Rust_Enclave_Name)

$(Rust_Enclave_Name): $(Rust_Enclave_Files)
ifeq ($(XARGO_SGX), 1)
	RUST_TARGET_PATH=$(Rust_Target_Path) xargo build --target x86_64-unknown-linux-sgx --release
	cp ./target/x86_64-unknown-linux-sgx/release/libsealdataenclave.a ../lib/libenclave.a
else
	$(CARGOTOOL) build --release
	@mkdir -p ../lib
	@cp ./target/release/$(Sgx_Library_Name) ../lib/libenclave.a
endif
