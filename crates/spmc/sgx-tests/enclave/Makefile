Rust_Enclave_Name := librun_tests_enclave.a
Rust_Enclave_Files := $(wildcard src/*.rs)
Rust_Target_Path := $(CURDIR)

.PHONY: all

all: $(Rust_Enclave_Name)

$(Rust_Enclave_Name): $(Rust_Enclave_Files)
ifeq ($(XARGO_SGX), 1)
	RUST_TARGET_PATH=$(Rust_Target_Path) xargo build --target x86_64-unknown-linux-sgx --release
	cp ./target/x86_64-unknown-linux-sgx/release/$(Rust_Enclave_Name) ../lib/$(Rust_Enclave_Name)
else
	cargo build --release
	cp ./target/release/$(Rust_Enclave_Name) ../lib/$(Rust_Enclave_Name)
endif
